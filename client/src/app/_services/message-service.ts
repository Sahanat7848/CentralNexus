import { HttpClient } from '@angular/common/http';
import { Injectable, inject } from '@angular/core';
import { firstValueFrom } from 'rxjs';
import { Message } from '../_models/message';
import { environment } from '../../environments/environment';

@Injectable({
    providedIn: 'root'
})
export class MessageService {
    private _http = inject(HttpClient);
    private _baseUrl = `${environment.baseUrl}`;

    async send_message(receiver_id: number, content: string): Promise<Message> {
        return firstValueFrom(this._http.post<Message>(`${this._baseUrl}/messages/send`, { receiver_id, content }));
    }

    async get_conversation(friend_id: number): Promise<Message[]> {
        return firstValueFrom(this._http.get<Message[]>(`${this._baseUrl}/messages/${friend_id}`));
    }
}
