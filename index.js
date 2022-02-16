
import { newClient, connect, play } from './native';

export default class Spotify {
  native;

  constructor() {
    this.native = newClient();
  }

  connect(username, oauth) {
    connect(this.native, username, oauth)
  }


    async play(trackId) {
    play(this.native, trackId);
  }

    async pause() {
    // this.native.
  }

    async stop() {

  }

    async seek(positionMs) {

  }

    async getPosition(){
    return 0;
  }

    async getCurrentTrack(){
    // TODO: use property instead?
    return ''
  }

    async isPlaying() {
    return false;
  }

    async teardown() {

  }


}