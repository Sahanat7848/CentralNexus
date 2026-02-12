import { computed, inject, Injectable, signal } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { firstValueFrom, Observable } from 'rxjs';
import { environment } from '../../environments/environment';
import { LoginModel, Passport, RegisterModel } from '../_models/passport';

import { getAvatar } from '../_helpers/avatar';

@Injectable({
    providedIn: 'root'
})
export class PassportService {
    image = computed(() => getAvatar(this.data()));
    private _storage_key = 'passport'
    private _api_url = environment.baseUrl
    private _http: HttpClient = inject(HttpClient)

    data = signal<Passport | undefined>(undefined)
    avatar = signal<string>("")
    isSignin = signal<boolean>(false)

    saveAvatarImgUrl(url: string) {
        this.data.update(passport => {
            if (passport) {
                return { ...passport, avatar_url: url };
            }
            return passport;
        });
        this.savePassportToLocalStorage();
    }

    updateDisplayName(newName: string) {
        this.data.update(passport => {
            if (passport) {
                return { ...passport, display_name: newName };
            }
            return passport;
        });
        this.savePassportToLocalStorage();
    }

    constructor() {
        this.getPassportFromLocalStorage()
    }

    private getPassportFromLocalStorage(): void {
        const jsonStr = localStorage.getItem(this._storage_key)
        if (!jsonStr) return
        try {
            const passport: Passport = JSON.parse(jsonStr) as Passport
            this.data.set(passport)
            this.isSignin.set(true)
        } catch (error) {
            console.error(error)
        }
    }

    private savePassportToLocalStorage(): void {
        const passport = this.data()
        if (!passport) return
        const passportJson = JSON.stringify(passport)
        localStorage.setItem(this._storage_key, passportJson)
        this.isSignin.set(true)
    }

    async login(loginData: LoginModel): Promise<string> {
        try {
            const url = this._api_url + '/authentication/login'
            const source: Observable<Passport> = this._http.post<Passport>(url, loginData)
            const passport: Passport = await firstValueFrom(source)
            this.data.set(passport)
            this.savePassportToLocalStorage()
        } catch (error: any) {
            console.error(error)
            // Handle different error types
            if (error.error && typeof error.error === 'string') {
                return error.error
            } else if (error.message) {
                return error.message
            } else if (error.status === 0) {
                return 'Cannot connect to server'
            }
            return 'An error occurred'
        }
        return ''
    }

    destroy() {
        localStorage.removeItem(this._storage_key)
        this.data.set(undefined)
        this.isSignin.set(false)
    }

    async register(registerData: RegisterModel): Promise<string> {
        try {
            const url = this._api_url + '/brawlers/register'
            const source: Observable<Passport> = this._http.post<Passport>(url, registerData)
            const passport: Passport = await firstValueFrom(source)
            this.data.set(passport)

            this.savePassportToLocalStorage()
        } catch (error: any) {
            console.error(error)
            if (error.error && typeof error.error === 'string') {
                return error.error
            } else if (error.message) {
                return error.message
            } else if (error.status === 0) {
                return 'Cannot connect to server'
            }
            return 'An error occurred during registration'
        }
        return ''
    }
}