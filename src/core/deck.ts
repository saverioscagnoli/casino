import { createLabelsArray, createSuitsArray, createValuesArray, rng } from "~/lib/utils";
import { Card } from "~/core/card";
import * as THREE from "three";
import { mergeGeometries } from "three/examples/jsm/utils/BufferGeometryUtils.js";
import { Scene } from "~/core/scene";

class Deck {
  private cards: Card[];

  private movingCards: Card[];
  private rotatingCards: Card[];

  private constructor() {
    this.cards = [];
    this.movingCards = [];
    this.rotatingCards = [];
  }

  public static async create(): Promise<Deck> {
    const deck = new Deck();

    const suits = createSuitsArray();
    const labels = createLabelsArray();
    const values = createValuesArray();

    const backTexture = await Scene.loadTexture("/src/assets/cards/back-red.png");

    for (let i = 0; i < suits.length; i++) {
      for (let j = 0; j < labels.length; j++) {
        const suit = suits[i];
        const label = labels[j];
        const value = values[j];

        const frontTexture = await Scene.loadTexture(
          `/src/assets/cards/${label}-${suit}.png`
        );

        const cardFrontMaterial = new THREE.MeshBasicMaterial({
          map: frontTexture,
          transparent: true
        });

        const cardBackMaterial = new THREE.MeshBasicMaterial({
          map: backTexture,
          transparent: true
        });

        const cardMesh = new THREE.Mesh(
          mergeGeometries([Card.geometry, Card.geometry.clone().rotateY(Math.PI)], true),
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

  /**
   * Helper function to iterate over the cards
   * @returns {IterableIterator<[number, Card]>}
   */
  public iter() {
    return this.cards.entries();
  }

  public getCards(): Card[] {
    return this.cards;
  }

  public update() {
    const newMovingCards = [];
    const newRotatingCards = [];

    for (const card of this.movingCards) {
      if (card.isMoving()) {
        card.move(0.1);
        newMovingCards.push(card);
      }
    }

    for (const card of this.rotatingCards) {
      if (card.isRotating()) {
        card.rotate(0.1);
        newRotatingCards.push(card);
      }
    }

    this.movingCards = newMovingCards;
    this.rotatingCards = newRotatingCards;
  }

  public drawAt(index: number, x: number, y: number, z: number) {
    const card = this.cards.at(index);

    if (card === undefined) {
      return;
    }

    this.cards.splice(index, 1);

    this.movingCards.push(card);
    this.rotatingCards.push(card);

    card.setTargetPosition(x, y, z);
    card.setTargetRotation(1, 0, 0);
  }

  public draw(x: number, y: number, z: number) {
    this.drawAt(0, x, y, z);
  }
}

export { Deck };
