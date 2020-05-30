#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

struct CPair {
  const uint8_t *secret_data;
  uintptr_t secret_len;
  const uint8_t *peer_id_data;
  uintptr_t peer_id_len;
};

struct CEvent {
  enum class Tag {
    Message,
  };

  struct Message_Body {
    const uint8_t *_0;
    uintptr_t _1;
  };

  Tag tag;
  union {
    Message_Body message;
  };
};

extern "C" {

CPair generate_pair();

void start(void (*callback)(CEvent));

} // extern "C"
