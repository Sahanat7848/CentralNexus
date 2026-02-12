export interface Message {
    id: number;
    sender_id: number;
    receiver_id: number;
    content: String;
    created_at: Date;
    read_at?: Date;
}
