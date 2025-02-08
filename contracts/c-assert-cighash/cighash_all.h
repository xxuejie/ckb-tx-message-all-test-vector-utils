#ifndef CKB_CIGHASH_ALL_H_
#define CKB_CIGHASH_ALL_H_

#include <ckb_syscalls.h>

#include "witness_args_lazy_utils.h"

#ifndef CIGHASH_WITNESS_BUFFER_SIZE
#define CIGHASH_WITNESS_BUFFER_SIZE (1024 * 64)
#endif

#ifndef CIGHASH_CKB_LOAD_BUFFER_SIZE
#define CIGHASH_CKB_LOAD_BUFFER_SIZE (1024 * 64)
#endif

typedef int (*cighash_write_func_t)(const uint8_t* data, size_t length,
                                    void* context);
typedef int (*_cighash_ckb_load_func_t)(void* addr, uint64_t* len,
                                        size_t offset, size_t index,
                                        size_t source);

static int _cighash_load_and_hash(_cighash_ckb_load_func_t loader,
                                  int sign_length, size_t index, size_t source,
                                  cighash_write_func_t writer, void* context);

static int cighash_all_generate(cighash_write_func_t writer, void* context) {
  uint8_t
      first_witness_buffer[MOL2_DATA_SOURCE_LEN(CIGHASH_WITNESS_BUFFER_SIZE)];
  WitnessArgsType first_witness;
  /* Load and validate the first witness */
  int err = mol2_lazy_witness_args_load(
      first_witness_buffer, CIGHASH_WITNESS_BUFFER_SIZE, 0,
      CKB_SOURCE_GROUP_INPUT, 1, &first_witness);
  if (err != 0) {
    return err;
  }

  /* Hash tx hash */
  {
    uint8_t tx_hash[32];
    uint64_t len = 32;
    err = ckb_load_tx_hash(tx_hash, &len, 0);
    if (err != 0) {
      return err;
    }
    ASSERT(len == 32);
    err = writer(tx_hash, 32, context);
    if (err != 0) {
      return err;
    }
  }

  /* Hash all input cells */
  size_t input_cells = 0;
  {
    while (1) {
      err = _cighash_load_and_hash(ckb_load_cell, 0, input_cells,
                                   CKB_SOURCE_INPUT, writer, context);
      if (err == CKB_INDEX_OUT_OF_BOUND) {
        break;
      }
      if (err != 0) {
        return err;
      }

      err = _cighash_load_and_hash(ckb_load_cell_data, 1, input_cells,
                                   CKB_SOURCE_INPUT, writer, context);
      if (err != 0) {
        return err;
      }
      input_cells += 1;
    }
  }

  /* Hash the first witness in specified format */
  {
    uint8_t buffer[CIGHASH_CKB_LOAD_BUFFER_SIZE];
    {
      uint32_t read_len = mol2_read_at(&first_witness.cur, buffer, 16);
      if (read_len != 16) {
        MOL2_PANIC(MOL2_ERR_DATA);
        return MOL2_ERR_DATA;
      }
      err = writer(buffer, 16, context);
      if (err != 0) {
        return err;
      }
    }

    {
      mol2_cursor_t input_type =
          first_witness.t->input_type(&first_witness).cur;
      uint32_t read = 0;
      uint32_t total_size = input_type.size;
      while (read < total_size) {
        uint32_t current =
            mol2_read_at(&input_type, buffer, CIGHASH_CKB_LOAD_BUFFER_SIZE);
        ASSERT(current > 0);
        err = writer(buffer, current, context);
        if (err != 0) {
          return err;
        }
        mol2_add_offset(&input_type, current);
        mol2_sub_size(&input_type, current);
        read += current;
      }
    }

    {
      mol2_cursor_t output_type =
          first_witness.t->output_type(&first_witness).cur;
      uint32_t read = 0;
      uint32_t total_size = output_type.size;
      while (read < total_size) {
        uint32_t current =
            mol2_read_at(&output_type, buffer, CIGHASH_CKB_LOAD_BUFFER_SIZE);
        ASSERT(current > 0);
        err = writer(buffer, current, context);
        if (err != 0) {
          return err;
        }
        mol2_add_offset(&output_type, current);
        mol2_sub_size(&output_type, current);
        read += current;
      }
    }
  }

  /* Hash remaining witness in current script group */
  {
    size_t i = 1;
    while (1) {
      err = _cighash_load_and_hash(ckb_load_witness, 1, i,
                                   CKB_SOURCE_GROUP_INPUT, writer, context);
      if (err == CKB_INDEX_OUT_OF_BOUND) {
        break;
      }
      if (err != 0) {
        return err;
      }
      i += 1;
    }
  }

  /* Hash the witnesses which do not have input cells of matching indices */
  {
    size_t i = input_cells;

    while (1) {
      err = _cighash_load_and_hash(ckb_load_witness, 1, i, CKB_SOURCE_INPUT,
                                   writer, context);
      if (err == CKB_INDEX_OUT_OF_BOUND) {
        break;
      }
      if (err != 0) {
        return err;
      }
      i += 1;
    }
  }

  return 0;
}

static int _cighash_load_and_hash(_cighash_ckb_load_func_t loader,
                                  int sign_length, size_t index, size_t source,
                                  cighash_write_func_t writer, void* context) {
  uint8_t buffer[CIGHASH_CKB_LOAD_BUFFER_SIZE];
  uint64_t total_length = CIGHASH_CKB_LOAD_BUFFER_SIZE;

  int err = loader(buffer, &total_length, 0, index, source);
  if (err != 0) {
    return err;
  }

  if (sign_length != 0) {
    uint32_t length_data = (uint32_t)total_length;
    err = writer((uint8_t*)&length_data, 4, context);
    if (err != 0) {
      return err;
    }
  }

  uint64_t loaded = total_length;
  if (loaded > CIGHASH_CKB_LOAD_BUFFER_SIZE) {
    loaded = CIGHASH_CKB_LOAD_BUFFER_SIZE;
  }
  err = writer(buffer, loaded, context);
  if (err != 0) {
    return err;
  }

  while (loaded < total_length) {
    uint64_t length = CIGHASH_CKB_LOAD_BUFFER_SIZE;
    err = loader(buffer, &length, loaded, index, source);
    if (err != 0) {
      return err;
    }
    ASSERT(length > 0);
    if (length > CIGHASH_CKB_LOAD_BUFFER_SIZE) {
      length = CIGHASH_CKB_LOAD_BUFFER_SIZE;
    }
    err = writer(buffer, length, context);
    if (err != 0) {
      return err;
    }
    loaded += length;
  }

  return 0;
}

#endif /* CKB_CIGHASH_ALL_H_ */
