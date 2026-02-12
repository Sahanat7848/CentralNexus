import { inject, Injectable } from '@angular/core';
import { environment } from '../../environments/environment';
import { HttpClient } from '@angular/common/http';
import { PassportService } from './passport-service';
import { fileToBase64 } from '../_helpers/file';
import { firstValueFrom } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class UserService {
  private _base_url = environment.baseUrl + '/brawlers'
  private _http = inject(HttpClient)
  private _passport = inject(PassportService)

  async uploadAvatarImg(file: File): Promise<string | null> {
    const url = this._base_url + '/avatar';
    const base64string = await fileToBase64(file);
    const uploadImg = {
      'base64_string': base64string.split(',')[1]
    };
    try {
      const clodinaryImg = await firstValueFrom(
        this._http.post<{ url: string }>(url, uploadImg)
      );
      this._passport.saveAvatarImgUrl(clodinaryImg.url);
    } catch (error: any) {
      return error.error as string;
    }
    return null;
  }
  async updateDisplayName(newName: string): Promise<string | null> {
    const url = this._base_url + '/update-name';
    try {
      await firstValueFrom(this._http.post(url, { display_name: newName }, { responseType: 'text' }));
      this._passport.updateDisplayName(newName);
      return null;
    } catch (error: any) {
      console.error('Update display name error:', error);
      if (typeof error.error === 'string') {
        return error.error;
      }
      return error.message || 'Failed to update name';
    }
  }
}
