export interface Brawler {
    display_name: string;
    avatar_url: string;
    mission_success_count: number;
    mission_joined_count: number;
}

export interface BrawlerSummary {
    id: number;
    display_name: string;
    tag: string;
    avatar_url: string | null;
}