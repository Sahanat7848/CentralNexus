import { MissionStatus } from "./mission-filter";

export interface Mission {
    id: number;
    name: string;
    description?: string;
    status: MissionStatus;
    chief_id: number;
    chief_display_name: string;
    crew_count: number;
    created_at: Date;
    updated_at: Date;
    // Client-side only properties
    is_joined?: boolean;
    is_chief?: boolean;
}
