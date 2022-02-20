
const { newPlayer, connectOauth, play, stop, pause, discoveryEnable, newDiscovery, playerReplaceEventListener } = require('./native');
const { EventEmitter } = require('events');

class Player extends EventEmitter {
  native

  constructor(nativePlayer) {
    super();
    this.native = (nativePlayer !== undefined) ? nativePlayer: newPlayer((e) => this.listenEvent(e));
  }

  listenEvent(event) {
    // console.log('TRIGGERED', event);
    this.emit(event, true);
  }

  connectOauth(username, oauth) {
    return new Promise((resolve, reject) => {
      connectOauth(this.native, username, oauth, (err, success) => {
        if (err !== undefined) {
          reject(err);
          return; 
        }
        resolve(success);
      })
    })
  }


  play(trackId, waitForEnd = false) {
    return new Promise((resolve, reject) => {
      play(this.native, trackId, waitForEnd, (err, success) => {
        if (err !== undefined) {
          reject(err);
          return; 
        }
        resolve(success);
      })
    })
  }

    async pause() {
      return new Promise((resolve, reject) => {
        pause(this.native, (err, success) => {
          if (err !== undefined) {
            reject(err);
            return; 
          }
          resolve(success);
        })
      })
  }

    async stop() {
      return new Promise((resolve, reject) => {
        stop(this.native, (err, success) => {
          if (err !== undefined) {
            reject(err);
            return; 
          }
          resolve(success);
        })
      })
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

class Discover extends EventEmitter {
  native

  constructor() {
    super();
    this.native = newDiscovery((e, d) => this.listenEvent(e, d));
  }

  listenEvent(event, data) {
    if (event === "discovered_player") {
      console.log('CREATE NEW PLAYER');
      data = new Player(data);
      
      playerReplaceEventListener(data.native, (e) => data.listenEvent(e))
      
      console.log('CREATE NEW PLAYER FINISHED');

    }
    this.emit(event, data);
  }

  async enable(name, device = "speaker"){
    return new Promise((resolve, reject) => {
      discoveryEnable(this.native, name, device, (err, success) => {
        if (err !== undefined) {
          reject(err);
          return; 
        }
        resolve(success);
      })
    })
  }

}


module.exports = { Player, Discover }
