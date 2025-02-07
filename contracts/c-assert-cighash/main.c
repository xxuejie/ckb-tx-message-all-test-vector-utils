#include <blake2b.h>

#include "cighash_all.h"

#define WITNESS_BUFFER_SIZE (1024 * 32)

int write_to_blake2b(const uint8_t* data, size_t length, void* context) {
  blake2b_state* state = (blake2b_state*)context;
  blake2b_update(state, data, length);
  return 0;
}

int main() {
  blake2b_state state;
  ckb_blake2b_init(&state, 32);

  int err = cighash_all_generate(write_to_blake2b, &state);
  if (err != 0) {
    ckb_printf("CIGHASH_ALL encounters error: %d\n", err);
    return 99;
  }

  uint8_t first_witness_buffer[MOL2_DATA_SOURCE_LEN(WITNESS_BUFFER_SIZE)];
  WitnessArgsType first_witness;
  /* We can skip the validation since CIGHASH_ALL process takes care of it */
  err =
      mol2_lazy_witness_args_load(first_witness_buffer, WITNESS_BUFFER_SIZE, 0,
                                  CKB_SOURCE_GROUP_INPUT, 0, &first_witness);
  if (err != 0) {
    ckb_printf("Loading the first witness encounters error: %d\n", err);
    return err;
  }

  BytesOptType lock = first_witness.t->lock(&first_witness);
  if (lock.t->is_none(&lock)) {
    ckb_printf("Lock field in WitnessArgs is lacking content!\n");
    return -1;
  }

  mol2_cursor_t lock_content = lock.t->unwrap(&lock);
  if (lock_content.size != 32) {
    ckb_printf("Lock field has length: %u, which is expected to be 32!\n",
               lock_content.size);
    return -1;
  }

  uint8_t expected[32];
  uint32_t len = mol2_read_at(&lock_content, expected, 32);
  if (len != 32) {
    ckb_printf("Read %u bytes of data from lock, which is expected to be 32!\n",
               len);
    return -1;
  }

  uint8_t actual[32];
  blake2b_final(&state, actual, 32);

  if (memcmp(actual, expected, 32) != 0) {
    ckb_printf("CIGHASH_ALL does not match!\n");
    return -1;
  }

  return 0;
}
