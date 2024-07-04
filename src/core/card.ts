import * as THREE from "three";

type CardSuit = "H" | "D" | "S" | "C";
type CardLabel =
  | "A"
  | "2"
  | "3"
  | "4"
  | "5"
  | "6"
  | "7"
  | "8"
  | "9"
  | "10"
  | "J"
  | "Q"
  | "K";

class Card {
  public static geometry: THREE.PlaneGeometry = new THREE.PlaneGeometry(2, 3.5);

  private suit: CardSuit;
  private label: CardLabel;
  private value: number;

  public mesh: THREE.Mesh;

  public constructor(
    suit: CardSuit,
    label: CardLabel,
    value: number,
    mesh: THREE.Mesh
  ) {
    this.suit = suit;
    this.label = label;
    this.value = value;
    this.mesh = mesh;
  }

  public getSuit(): CardSuit {
    return this.suit;
  }

  public getLabel(): CardLabel {
    return this.label;
  }

  public getValue(): number {
    return this.value;
  }
}

export { Card };
export type { CardSuit, CardLabel };
