enum Opcode {
  Hello = 0,
}

type Payload<T> = {
  op: Opcode;
  d: T;
};

type Hello = {
  username: string;
};

export { Opcode };
export type { Payload, Hello };
