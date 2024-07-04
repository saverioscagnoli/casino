import * as THREE from "three";

async function loadTexture(loader: THREE.TextureLoader, src: string) {
  return new Promise<THREE.Texture>((res, rej) => {
    loader.load(src, res, undefined, rej);
  });
}

function rng(min: number, max: number) {
  return (
    (window.crypto.getRandomValues(new Uint32Array(1))[0] % (max - min)) + min
  );
}

export { loadTexture, rng };
