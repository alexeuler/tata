#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * An enum representing the available verbosity level filters of the logger.
 *
 * A `LevelFilter` may be compared directly to a [`Level`]. Use this type
 * to get and set the maximum log level with [`max_level()`] and [`set_max_level`].
 *
 * [`Level`]: enum.Level.html
 * [`max_level()`]: fn.max_level.html
 * [`set_max_level`]: fn.set_max_level.html
 */
enum LevelFilter {
  /**
   * A level lower than all log levels.
   */
  Off,
  /**
   * Corresponds to the `Error` log level.
   */
  Error,
  /**
   * Corresponds to the `Warn` log level.
   */
  Warn,
  /**
   * Corresponds to the `Info` log level.
   */
  Info,
  /**
   * Corresponds to the `Debug` log level.
   */
  Debug,
  /**
   * Corresponds to the `Trace` log level.
   */
  Trace,
};
typedef uintptr_t LevelFilter;

/**
 * FFI representation of array of bytes
 *
 * The memory in the array should be dropped manually (using `free` method).
 * The Into<_> traits frees the memory automatically
 */
typedef struct {
  uint8_t *data;
  uintptr_t len;
} ByteArray;

/**
 * FFI representation of KeyPair
 */
typedef struct {
  ByteArray secret;
  ByteArray peer_id;
} KeyPair;

/**
 * Alias for level from `log` crate
 */
typedef LevelFilter LogLevel;

/**
 * Free allocated ByteArray memory. This needs to be called e.g. after start function for `secret_array`
 * if you're using the library from C.
 */
void free_array(ByteArray array);

/**
 * Generate secret keypair (to derive PeerId, i.e. p2p identity)
 */
KeyPair generate_keypair(void);

/**
 * Send a message to peer.
 *
 * ## Arguments
 *
 * `to_peer_id` - base58 Libp2p peer_id. This one is taken from discovery events from `start_network`.
 *
 * `message` - utf8 text content of the message
 *
 * `timestamp` - unix timestamp, essentially an id of the message
 */
bool send_message(ByteArray to_peer_id,
                  ByteArray message,
                  uint64_t timestamp);

/**
 * Starts the networking process in the background.
 * ## Arguments
 *
 *
 * `secret_array` - a Sec256k1 private key bytes
 *
 * `name` - your name as seen to other peers
 *
 * `callback` - triggered on any event with bytes representing
 * serialized json event (`primitives::PeerEvent`).
 *
 * `enable_logs` - enables or disables logs
 *
 * `log_level` - the level of the log
 */
bool start_network(ByteArray secret_array,
                   ByteArray name,
                   void (*callback)(ByteArray),
                   bool enable_logs,
                   LogLevel log_level);
