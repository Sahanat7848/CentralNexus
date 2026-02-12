import { inject, Injectable } from '@angular/core';
import { environment } from '../../environments/environment';
import { HttpClient } from '@angular/common/http';
import { MissionFilter } from '../_models/mission-filter';
import { firstValueFrom } from 'rxjs';
import { Mission } from '../_models/mission';
import { AddMission } from '../_models/add-mission';

@Injectable({
  providedIn: 'root',
})
export class MissionService {
  private _api_url = environment.baseUrl;
  private _http = inject(HttpClient)

  filter: MissionFilter = {}

  async get_all(filter: MissionFilter): Promise<Mission[]> {
    const queryString = this.createQueryString(filter);
    const url = this._api_url + '/view/filter' + (queryString ? '?' + queryString : '');
    console.log('Fetching missions from:', url);
    return await firstValueFrom(this._http.get<Mission[]>(url));
  }

  async getByFilter(filter: MissionFilter): Promise<Mission[]> {
    return await this.get_all(filter);
  }
  async add(mission: AddMission): Promise<number> {
    const url = this._api_url + '/missions/create';
    const observable = this._http.post<{ mission_id: number }>(url, mission);
    const resp = await firstValueFrom(observable);
    return resp.mission_id;
  }

  private createQueryString(filter: MissionFilter) {
    this.filter = filter
    const params: string[] = []

    if (filter.name && filter.name.trim()) {
      params.push(`name=${encodeURIComponent(filter.name.trim())}`)
    }
    if (filter.status) {
      params.push(`status=${encodeURIComponent(filter.status)}`)
    }
    if (filter.brawler_id) {
      params.push(`brawler_id=${filter.brawler_id}`)
    }

    return params.join('&');
  }
  async getMyMissions(): Promise<Mission[]> {
    const url = this._api_url + '/brawlers/my-missions';
    const observable = this._http.get<Mission[]>(url);
    const resp = await firstValueFrom(observable);
    return resp;
  }

  async joinMission(missionId: number): Promise<void> {
    const url = `${this._api_url}/crew_operation/join/${missionId}`;
    await firstValueFrom(this._http.post(url, {}));
  }

  async leaveMission(missionId: number): Promise<void> {
    const url = `${this._api_url}/crew_operation/leave/${missionId}`;
    await firstValueFrom(this._http.delete(url));
  }
}
