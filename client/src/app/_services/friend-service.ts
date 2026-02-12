import { inject, Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { environment } from '../../environments/environment';
import { firstValueFrom } from 'rxjs';
import { BrawlerSummary } from '../_models/brawler';

@Injectable({
    providedIn: 'root',
})
export class FriendService {
    private _base_url = environment.baseUrl + '/friends';
    private _http = inject(HttpClient);

    async searchFriend(query: string): Promise<BrawlerSummary> {
        const url = `${this._base_url}/search`;
        const params = new HttpParams().set('query', query);
        return firstValueFrom(this._http.get<BrawlerSummary>(url, { params }));
    }

    async addFriend(friendId: number): Promise<void> {
        const url = `${this._base_url}/add`;
        await firstValueFrom(this._http.post(url, { friend_id: friendId }, { responseType: 'text' }));
    }

    async getFriends(): Promise<BrawlerSummary[]> {
        return firstValueFrom(this._http.get<BrawlerSummary[]>(this._base_url));
    }
}
