import { Scene } from "~/core/scene";
import { Card } from "~/game/card";
import { createLabelsArray, createSuitsArray, createValuesArray, rng } from "~/lib/utils";
import { Player } from "~/game/player";
import * as THREE from "three";

type DeckArgs = {
  scene: Scene;
  shuffled: boolean;
};

class Deck {
  public cards: Card[];
  private dealtCards: number;

  public constructor() {
    this.cards = [];
    this.dealtCards = 0;
  }

  public init({ scene, shuffled }: DeckArgs) {
    const suits = createSuitsArray();
    const labels = createLabelsArray();
    const values = createValuesArray();

    for (let i = 0; i < suits.length; i++) {
      for (let j = 0; j < labels.length; j++) {
        const suit = suits[i];
        const label = labels[j];
        const value = values[j];

        const card = new Card(suit, label, value);
        this.cards.push(card);

        scene.add(card.mesh);
      }
    }

    if (shuffled) {
      this.shuffle();
    }
  }

  public shuffle() {
    for (let i = this.cards.length - 1; i > 0; i--) {
      const j = Math.floor(rng(0, i + 1));
      [this.cards[i], this.cards[j]] = [this.cards[j], this.cards[i]];
    }
  }

  // Places the cards in order on the top right corner of the table
  public placeInShuffler() {
    for (const [i, card] of this.cards.entries()) {
      card.mesh.position.set(18 + i * 0.1, 1, -8);

      card.mesh.rotation.x = -Math.PI / 2;
      card.mesh.rotation.y = Math.PI / 2;
    }
  }

  public draw() {
    return this.cards[this.dealtCards++];
  }

  public dealTo(player: Player, hidden: boolean = false): Promise<void> {
    const card = this.draw();

    card.setTargetPosition(player.getNextCardPosition());

    if (hidden) {
      card.hide();
    } else {
      card.show();
    }

    if (player.isDealer()) {
      card.mesh.scale.set(1.3, 1.3, 1.3);
    }

    return new Promise(res => {
      player.addCard(card);

      card.on("arrived", () => {
        if (player.isDealer() && !hidden) {
          card.setTargetPosition(
            new THREE.Vector3(card.mesh.position.x, 1, card.mesh.position.z)
          );

          card.setFloating(true);
          card.setTargetRotation(new THREE.Vector3(Math.PI / 4, 0, 0));
        }

        res();
      });
    });
  }

  public async dealToAll(
    players: Player[],
    toDealer: boolean = false,
    hidden: boolean = false
  ) {
    for (const player of players) {
      if (player.isDealer()) {
        continue;
      }

      await this.dealTo(player, hidden);
    }

    if (toDealer) {
      const dealer = players.find(player => player.isDealer());
      await this.dealTo(dealer!, hidden);
    }
  }

  public update() {
    for (const card of this.cards) {
      card.update();
    }
  }
}

export { Deck };
