export class Player extends NodeJS.EventEmitter {
  connectOauth(username: string, oauth: string): Promise<void>;
  play(track: string): Promise<boolean>;
  pause(): Promise<void>;
  stop(): Promise<void>;
}