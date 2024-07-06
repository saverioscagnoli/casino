import { CardLabel, CardSuit } from "~/lib/enums";

function createSuitsArray(): CardSuit[] {
  return [CardSuit.Hearts, CardSuit.Diamonds, CardSuit.Spades, CardSuit.Clubs];
}

function createLabelsArray(): CardLabel[] {
  return [
    CardLabel.Ace,
    CardLabel.Two,
    CardLabel.Three,
    CardLabel.Four,
    CardLabel.Five,
    CardLabel.Six,
    CardLabel.Seven,
    CardLabel.Eight,
    CardLabel.Nine,
    CardLabel.Ten,
    CardLabel.Jack,
    CardLabel.Queen,
    CardLabel.King
  ];
}

function createValuesArray(): number[] {
  return [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10];
}

function rng(min: number, max: number) {
  return (window.crypto.getRandomValues(new Uint32Array(1))[0] % (max - min)) + min;
}

function getCardName(suit: CardSuit, label: CardLabel) {
  return `${label}-${suit}`;
}

export { createSuitsArray, createLabelsArray, createValuesArray, rng, getCardName };
