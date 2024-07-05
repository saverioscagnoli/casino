import { CardLabel, CardSuit } from "~/core/card";

function createSuitsArray(): CardSuit[] {
  return ["H", "D", "S", "C"];
}

function createLabelsArray(): CardLabel[] {
  return ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
}

function createValuesArray(): number[] {
  return [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10];
}

function rng(min: number, max: number) {
  return (window.crypto.getRandomValues(new Uint32Array(1))[0] % (max - min)) + min;
}

export { createSuitsArray, createLabelsArray, createValuesArray, rng };
