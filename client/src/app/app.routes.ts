import { Routes } from '@angular/router';
import { Home } from './home/home';
import { Login } from './login/login';
import { Profile } from './profile/profile';
import { ServerError } from './server-error/server-error';
import { authGuard } from './_guard/auth.guard';
import { guestGuard } from './_guard/guest.guard';
import { Missions } from './missions/missions';
import { MissionManager } from './mission/mission-manager/mission-manager';
import { Friends } from './friends/friends';

export const routes: Routes = [
    { path: '', component: Home, canActivate: [authGuard] },
    { path: 'login', component: Login, canActivate: [guestGuard] },
    { path: 'profile', component: Profile, canActivate: [authGuard], runGuardsAndResolvers: 'always' },
    { path: 'missions', component: Missions, canActivate: [authGuard], runGuardsAndResolvers: 'always' },
    { path: 'friends', component: Friends, canActivate: [authGuard] },
    { path: 'server-error', component: ServerError },
    {
        path: 'my-missions',
        component: MissionManager,
        runGuardsAndResolvers: 'always',
        canActivate: [authGuard]
    }
];