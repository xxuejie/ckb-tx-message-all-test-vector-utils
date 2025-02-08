use ckb_gen_types::{packed::WitnessArgsReader, prelude::*};
use ckb_rust_std::io;
use ckb_std::{
    ckb_constants::Source,
    error::SysError,
    high_level::{self, QueryIter},
    syscalls,
};
use molecule::error::VerificationError;

#[derive(Debug)]
#[allow(dead_code)]
pub enum CighashAllError {
    Witness(VerificationError),
    Syscall(SysError),
    Io(io::Error),
}

impl From<VerificationError> for CighashAllError {
    fn from(e: VerificationError) -> Self {
        CighashAllError::Witness(e)
    }
}

impl From<SysError> for CighashAllError {
    fn from(e: SysError) -> Self {
        CighashAllError::Syscall(e)
    }
}

impl From<io::Error> for CighashAllError {
    fn from(e: io::Error) -> Self {
        CighashAllError::Io(e)
    }
}

pub fn generate_cighash_all<W: io::Write>(writer: &mut W) -> Result<(), CighashAllError> {
    // NOTE: while the first step in CIGHASH_ALL's specification is to validate
    // the format of the first witness in current script group, the actual validation
    // code is shifted to a later stage due to certain reasons we will explain later.
    // The actual semantics stay the same, an invalid witness would still generate an
    // error, with the side effect that a few extra bytes have been fed into writer structure.

    // Hash tx hash
    writer.write_all(&high_level::load_tx_hash()?)?;

    // Hash contents of all input cells
    let cell_output_iter = QueryIter::new(
        |index, source| load_initial(syscalls::load_cell, index, source),
        Source::Input,
    );
    let cell_data_iter = QueryIter::new(
        |index, source| load_initial(syscalls::load_cell_data, index, source),
        Source::Input,
    );
    let mut input_cell_count = 0;
    for (initial_cell_output, initial_cell_data) in cell_output_iter.zip(cell_data_iter) {
        input_cell_count += 1;

        load_and_hash(initial_cell_output, syscalls::load_cell, writer)?;

        write_length(initial_cell_data.full_length, writer)?;
        load_and_hash(initial_cell_data, syscalls::load_cell_data, writer)?;
    }

    // Hash the first witness of current script group
    {
        // Theoretically, a witness can be almost as large as a CKB block, which
        // is roughly 600K. The old way would be loading a witness as a whole into
        // memory, however a VM instance only has 4M memory, shared by code and all
        // data. Loading a buffer which can be as large as 600K, will put unnecessary
        // pressure on memory. Which is why a special +load_and_hash+ function is
        // implemented, so we load cell outputs, cell data and other witnesses in
        // fixed-length batches. But the situation is slightly different for the first
        // witness of the current script group: we will need to validate its own strcture,
        // meaning we will selectively read certain bytes of the witness field, instead
        // of sequentially reading everything. In addition, certain bytes of the witness
        // needs to be zero filled when it is hashed. While a lazy reader has been
        // introduced to Rust's molecule API so only needed part can be loaded into
        // the memory, maintaining a constant usage of memory, the lazy reader lacks
        // support to validate a molecule structure.
        //
        // So for the moment, we use a tradeoff so we load the first witness of the
        // current script group still as a single buffer. This part is grouped in its
        // own block hoping Rust decided that the used memory can be freed up as soon
        // as the processing is completed. It is also why we move the validation of the
        // first witness here so all the code requiring the first witness can be grouped
        // together.
        //
        // A different solution also exists: one can manually code the validator for
        // WitnessArgs alone on the lazy reader API. However, such a validator would
        // peek into internal data structure of the lazy reader API(e.g., we need to
        // know the length of a cursor structure). It remains a debate which solution
        // is a more proper one.
        let first_witness_data = high_level::load_witness(0, Source::GroupInput)?;
        let first_witness = WitnessArgsReader::from_slice(&first_witness_data)?;

        writer.write_all(&first_witness.as_slice()[0..16])?;
        writer.write_all(first_witness.input_type().as_slice())?;
        writer.write_all(first_witness.output_type().as_slice())?;
    }

    // Hash the remaining witnesses in current script group
    for initial_witness in QueryIter::new(
        |index, source| load_initial(syscalls::load_witness, index, source),
        Source::GroupInput,
    )
    .skip(1)
    {
        write_length(initial_witness.full_length, writer)?;
        load_and_hash(initial_witness, syscalls::load_witness, writer)?;
    }

    // Hash witnesses which do not have input cells of matching indices
    for initial_witness in QueryIter::new(
        |index, source| load_initial(syscalls::load_witness, index, source),
        Source::Input,
    )
    .skip(input_cell_count)
    {
        write_length(initial_witness.full_length, writer)?;
        load_and_hash(initial_witness, syscalls::load_witness, writer)?;
    }

    writer.flush()?;
    Ok(())
}

const LOAD_BATCH_LENGTH: usize = 32 * 1024;

struct InitialLoadData {
    index: usize,
    source: Source,
    full_length: usize,
    buffer: [u8; LOAD_BATCH_LENGTH],
}

fn load_initial<F>(load_fn: F, index: usize, source: Source) -> Result<InitialLoadData, SysError>
where
    F: Fn(&mut [u8], usize, usize, Source) -> Result<usize, SysError>,
{
    let mut buffer = [0u8; LOAD_BATCH_LENGTH];
    let full_length = match load_fn(&mut buffer, 0, index, source) {
        Ok(actual_length) => actual_length,
        Err(SysError::LengthNotEnough(actual_length)) => actual_length,
        Err(e) => return Err(e),
    };
    Ok(InitialLoadData {
        index,
        source,
        full_length,
        buffer,
    })
}

fn load_and_hash<W, F>(
    initial: InitialLoadData,
    load_fn: F,
    writer: &mut W,
) -> Result<(), CighashAllError>
where
    W: io::Write,
    F: Fn(&mut [u8], usize, usize, Source) -> Result<usize, SysError>,
{
    let InitialLoadData {
        full_length,
        index,
        source,
        mut buffer,
    } = initial;
    let mut loaded = if full_length > LOAD_BATCH_LENGTH {
        LOAD_BATCH_LENGTH
    } else {
        full_length
    };
    writer.write_all(&buffer[0..loaded])?;

    while loaded < full_length {
        match load_fn(&mut buffer, loaded, index, source) {
            Ok(current_loaded) => {
                assert!(loaded.checked_add(current_loaded).expect("overflow") == full_length);
                writer.write_all(&buffer[0..current_loaded])?;
                loaded += current_loaded;
            }
            Err(SysError::LengthNotEnough(_)) => {
                assert!(loaded.checked_add(LOAD_BATCH_LENGTH).expect("overflow") < full_length);
                writer.write_all(&buffer)?;
                loaded += LOAD_BATCH_LENGTH;
            }
            Err(e) => return Err(e.into()),
        }
    }

    Ok(())
}

#[inline]
fn write_length<W>(length: usize, writer: &mut W) -> Result<(), CighashAllError>
where
    W: io::Write,
{
    let length: u32 = length.try_into().expect("convert to u32");
    writer.write_all(&length.to_le_bytes())?;
    Ok(())
}
