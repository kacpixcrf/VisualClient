export interface Server {
  name: string;
  ip: string;
  accept_textures?: number;
  motdHtml?: string;
  loadingMotd?: boolean;
  online?: boolean;
  icon_base64?: string;
}

export interface World {
  folder_name: string;
  name: string;
  last_played: number;
  icon_base64?: string;
}

export interface EditServerForm {
  originalIp: string;
  name: string;
  ip: string;
  acceptTextures: number | null;
}
