import { loadTexture, rng } from "~/lib/utils";
import { Card, CardLabel, CardSuit } from "./card";
import * as THREE from "three";
import { mergeGeometries } from "three/examples/jsm/utils/BufferGeometryUtils.js";

class Deck {
  private cards: Card[];

  private constructor() {
    this.cards = [];
  }

  public static async create(): Promise<Deck> {
    const deck = new Deck();

    const suits: CardSuit[] = ["H", "D", "S", "C"];
    const labels: CardLabel[] = [
      "A",
      "2",
      "3",
      "4",
      "5",
      "6",
      "7",
      "8",
      "9",
      "10",
      "J",
      "Q",
      "K",
    ];

    const values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 10];

    const textureLoader = new THREE.TextureLoader();

    const backTexture = await loadTexture(
      textureLoader,
      "/src/assets/cards/back-red.png"
    );

    for (let i = 0; i < suits.length; i++) {
      for (let j = 0; j < labels.length; j++) {
        const suit = suits[i];
        const label = labels[j];
        const value = values[j];

        const frontTexture = await loadTexture(
          textureLoader,
          `/src/assets/cards/${label === "A" ? "1" : label}-${suit}.png`
        );

        const cardFrontMaterial = new THREE.MeshBasicMaterial({
          map: frontTexture,
          transparent: true,
        });

        const cardBackMaterial = new THREE.MeshBasicMaterial({
          map: backTexture,
          transparent: true,
        });

        const cardMesh = new THREE.Mesh(
          mergeGeometries(
            [Card.geometry, Card.geometry.clone().rotateY(Math.PI)],
            true
          ),
          [cardFrontMaterial, cardBackMaterial]
        );

        const card = new Card(suit, label, value, cardMesh);
        deck.cards.push(card);
      }
    }

    return deck;
  }

  public shuffle(): void {
    for (let i = this.cards.length - 1; i > 0; i--) {
      const j = Math.floor(rng(0, i + 1));
      [this.cards[i], this.cards[j]] = [this.cards[j], this.cards[i]];
    }
  }

  public getCards(): Card[] {
    return this.cards;
  }
}

export { Deck };
