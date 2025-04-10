import { TextOpcode } from "./opcdode";

/**
 * Generic type used to type any variable
 * before parsing the actual payload
 */
type UnknownTextPayload = {
  /**
   * Opcode of the json message
   */
  opcode: TextOpcode;

  /**
   * Actual underlying data
   */
  data: unknown;
};

type HelloPayload = {
  /**
   * The id that the backend is assigning to the client.
   */
  id: string;
};

type ChatMessagePayload = {
  /**
   * The id of the sender
   */
  id: string;

  /**
   * The text content of the message
   */
  content: string;
};

export type { UnknownTextPayload, HelloPayload, ChatMessagePayload };
