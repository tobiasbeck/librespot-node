const { Discover } = require('./')
//console.log(Spotify)

async function test_spot() {
  let discover = new Discover();

//console.log(spotify.native);
setTimeout(async () => {
discover.enable("me2");
discover.on('discovered_player', (player) => {
  // console.log('PLAYER!', player);
  player.play("spotify:track:5gPceIvoofOgu4s6FdsQc0")
})
setTimeout(() => console.log("done"), 90000)
}, 1000);
}

test_spot()
