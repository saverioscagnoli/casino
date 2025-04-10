/**
 * The opcode for json payloads
 * It is called `Text-` because of
 * coherence with the backend, where all string messages are
 * received as text.
 */
enum TextOpcode {
  Hello = "0",
  ChatMessage = "1",
}

export { TextOpcode };
