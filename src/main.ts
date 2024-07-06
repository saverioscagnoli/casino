import { Scene } from "~/core/scene";

import cardFlipAudio from "~/assets/sounds/card-flip.mp3";

const scene = new Scene();

window.addEventListener("resize", () => {
  Scene.screenResolution.set(window.innerWidth, window.innerHeight);
  Scene.aspectRatio = Scene.screenResolution.x / Scene.screenResolution.y;

  scene.getRenderer().setSize(Scene.screenResolution.x, Scene.screenResolution.y);

  scene.getCamera().aspect = Scene.aspectRatio;
  scene.getCamera().updateProjectionMatrix();
});


window.addEventListener("DOMContentLoaded", async () => {
  let animationID: number | null = null;

  function start() {
    if (animationID !== null) return;

    const loop = () => {
      scene.update();
      scene.render();

      animationID = requestAnimationFrame(loop);
    };

    loop();
  }

  function stop() {
    if (animationID === null) return;

    cancelAnimationFrame(animationID);
    animationID = null;
  }

  await scene.load();

  start();
});
