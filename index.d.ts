export class Spotify {
  connect(username: string, oauth: string);
  play(track: string);
  enableDiscovery(name: string, deviceType?: string);
}