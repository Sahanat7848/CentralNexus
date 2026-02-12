import { Component, inject } from '@angular/core';
import { Router } from '@angular/router';
import { MatIconModule } from '@angular/material/icon';
import { MatButtonModule } from '@angular/material/button';
import { RouterModule } from '@angular/router';

@Component({
  selector: 'app-server-error',
  imports: [MatIconModule, MatButtonModule, RouterModule],
  templateUrl: './server-error.html',
  styleUrl: './server-error.scss',
})
export class ServerError {
  private _router = inject(Router)
  errorMsg: string | undefined | null = ''

  constructor() {
    this.errorMsg = this._router.currentNavigation()?.extras.state?.['error'] as string
  }
}