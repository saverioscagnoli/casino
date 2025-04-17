import { create } from "zustand";
import { persist } from "zustand/middleware";

interface UserState {
  id: string;
  username: string;
  set: (id: string, username: string) => void;
}

const useUser = create(
  persist<UserState>(
    (set) => ({
      id: "",
      username: "",
      set: (id, username) => set({ id, username }),
    }),
    {
      name: "user",
    }
  )
);

export { useUser };
