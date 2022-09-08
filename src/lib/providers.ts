import { invoke } from "@tauri-apps/api";
import type {UpdateStatus} from "./updateStatus";

export class Provider {
    /// Provider id
    id: string;
    /// Tab caption
    caption: string;

    constructor(args: {id: string, caption: string}) {
        this.id = args.id;
        this.caption = args.caption;
    }


    public getPath(): string {
        return `/provider/${this.id}`
    }

    public async checkUpdate(): Promise<Array<UpdateStatus>> {
        return await invoke("check_update", { name: this.id});
    }
}

export const scoopProvider: Provider = new Provider(
    {
        id: 'scoop',
        caption: 'Scoop'
    }
);

export const providers = [
    scoopProvider
];

export function getProvider(id: string): Provider | null {
    return providers.find(p => p.id == id)
}
