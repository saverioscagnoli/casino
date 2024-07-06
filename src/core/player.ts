import * as THREE from "three";
import { Scene } from "~/core/scene";
import { Card } from "~/core/card";

class Player {
  private cardArea!: THREE.Mesh;
  private dealer: boolean;
  private hand: Card[];

  public constructor(isDealer: boolean) {
    this.dealer = isDealer;
    this.hand = [];
  }

  public init(scene: Scene, index: number, totalPlayers: number) {
    const cardAreaGeometry = new THREE.PlaneGeometry(5, 3);
    const cardAreaMaterial = new THREE.MeshBasicMaterial({
      transparent: true,
      opacity: 0
    });

    this.cardArea = new THREE.Mesh(cardAreaGeometry, cardAreaMaterial);

    if (index === -1) {
      this.cardArea.position.set(0, 10, -5);
    } else {
      // Calculate the total width of all card areas
      const totalWidth = totalPlayers * 5;

      // Calculate the starting x position
      const startX = -totalWidth / 2 + 2;

      // Set the position of this player's card area
      this.cardArea.position.set(startX + index * 5, -10, 5);
    }

    this.cardArea.rotation.x = -Math.PI / 2;

    scene.add(this.cardArea);
  }

  public isDealer() {
    return this.dealer;
  }

  public getHand() {
    return this.hand;
  }

  public getCardArea() {
    return this.cardArea;
  }

  public addCard(card: Card) {
    this.hand.push(card);
  }
}

export { Player };
