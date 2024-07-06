import * as THREE from "three";
import { Scene } from "~/core/scene";
import { Card } from "~/game/card";

class Player {
  private cardArea: THREE.Mesh;
  private hand: Card[];
  private dealer: boolean;

  public constructor({ dealer = false } = {}) {
    this.cardArea = new THREE.Mesh();
    this.hand = [];
    this.dealer = dealer;
  }

  public init(scene: Scene) {
    const cardAreaGeometry = new THREE.PlaneGeometry(3, 5);
    const cardAreaMaterial = new THREE.MeshBasicMaterial({
      opacity: 0,
      transparent: true
    });

    this.cardArea = new THREE.Mesh(cardAreaGeometry, cardAreaMaterial);
    this.cardArea.rotation.x = -Math.PI / 2;

    scene.add(this.cardArea);
  }

  public isDealer() {
    return this.dealer;
  }

  public setCardAreaPosition(x: number, y: number, z: number) {
    this.cardArea.position.set(x, y, z);
  }

  public addCard(card: Card) {
    this.hand.push(card);
  }

  // Returns the position where the next card should be placed
  // relative the the player's card area
  public getNextCardPosition() {
    const l = this.hand.length;

    return new THREE.Vector3(
      this.cardArea.position.x + l * 0.6,
      this.cardArea.position.y + l * 0.05,
      this.cardArea.position.z - 0.3
    );
  }
}

export { Player };
