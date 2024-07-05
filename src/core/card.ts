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
  private mesh: THREE.Mesh;

  private targetPosition: THREE.Vector3 | null;
  private targetRotation: THREE.Quaternion | null;

  public constructor(suit: CardSuit, label: CardLabel, value: number, mesh: THREE.Mesh) {
    this.suit = suit;
    this.label = label;
    this.value = value;
    this.mesh = mesh;

    this.targetPosition = null;
    this.targetRotation = null;
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

  public getMesh(): THREE.Mesh {
    return this.mesh;
  }

  public setTargetPosition(x: number, y: number, z: number): void {
    this.targetPosition = new THREE.Vector3(x, y, z);
  }

  public isMoving(): boolean {
    return this.targetPosition !== null;
  }

  public move(speed: number): void {
    if (this.targetPosition === null) {
      return;
    }

    if (this.mesh.position.distanceTo(this.targetPosition) < 0.01) {
      this.targetPosition = null;
      console.log("done");

      return;
    }

    this.mesh.position.lerp(this.targetPosition, speed);
  }

  public setTargetRotation(x: number, y: number, z: number): void {
    this.targetRotation = new THREE.Quaternion().setFromAxisAngle(
      new THREE.Vector3(x, y, z),
      -Math.PI / 2
    );
  }

  public isRotating(): boolean {
    return this.targetRotation !== null;
  }

  public rotate(speed: number): void {
    if (this.targetRotation === null) {
      return;
    }

    if (this.mesh.quaternion.angleTo(this.targetRotation) < 0.01) {
      this.targetRotation = null;
      return;
    }

    this.mesh.quaternion.slerp(this.targetRotation, speed);
  }
}

export { Card };
export type { CardSuit, CardLabel };
