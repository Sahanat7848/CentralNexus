import { Component, inject, OnDestroy, OnInit } from '@angular/core';
import { PassportService } from '../_services/passport-service';
import { Router } from '@angular/router';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-home',
  standalone: true,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule
  ],
  templateUrl: './home.html',
  styleUrl: './home.scss',
})
export class Home implements OnInit, OnDestroy {
  private _passport = inject(PassportService);
  private router = inject(Router);

  constructor() {
    if (!this._passport.data()) {
      this.router.navigate(['/login']);
    }
  }

  ngOnInit() {
  }

  navigate(path: string) {
    this.router.navigate([path]);
  }

  ngOnDestroy() {
  }
}