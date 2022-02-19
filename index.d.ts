export class Player {
  connectOauth(username: string, oauth: string): Promise<void>;
  play(track: string);
  pause(): Promise<void>;
  stop(): Promise<void>;
}