export interface MissionFilter {
    name?: string;
    status?: MissionStatus;
    brawler_id?: number;
}

export type MissionStatus = 'Open' | 'InProgress' | 'Completed' | 'Failed';
