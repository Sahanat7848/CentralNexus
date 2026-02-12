import { Component, computed, inject, signal, OnInit } from '@angular/core';
import { Router } from '@angular/router';
import { PassportService } from '../_services/passport-service';
import { MatDialog } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatCardModule } from '@angular/material/card';
import { MatTabsModule } from '@angular/material/tabs';
import { MatChipsModule } from '@angular/material/chips';
import { CommonModule } from '@angular/common';
import { UploadPhoto } from '../_dialog/upload-photo/upload-photo';
import { UserService } from '../_services/user-service';
import { MissionService } from '../_services/mission-service';
import { Mission } from '../_models/mission';
import { FormsModule } from '@angular/forms';
import { MatSnackBar, MatSnackBarModule } from '@angular/material/snack-bar';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';

@Component({
  selector: 'app-profile',
  standalone: true,
  imports: [
    CommonModule,
    MatButtonModule,
    MatIconModule,
    MatCardModule,
    MatTabsModule,
    MatChipsModule,
    FormsModule,
    MatSnackBarModule,
    MatInputModule,
    MatFormFieldModule
  ],
  templateUrl: './profile.html',
  styleUrl: './profile.scss'
})
export class Profile implements OnInit {
  private _passport = inject(PassportService);
  private _router = inject(Router);
  private _dialog = inject(MatDialog);
  private _userService = inject(UserService);
  private _missionService = inject(MissionService);
  private _snackBar = inject(MatSnackBar);

  displayName = computed(() => this._passport.data()?.display_name ?? 'Agent');
  tag = computed(() => this._passport.data()?.tag ?? '0000');
  avatarUrl = computed(() => this._passport.image());

  recentMissions = signal<Mission[]>([]);
  isLoadingMissions = signal(false);
  isEditingName = signal(false);
  tempName = signal('');

  // Dynamic stats based on actual data
  get stats() {
    const missions = this.recentMissions();
    const completed = missions.filter(m => m.status === 'Completed').length;
    const failed = missions.filter(m => m.status === 'Failed').length;
    const totalFinished = completed + failed;

    const successRate = totalFinished > 0 ? Math.round((completed / totalFinished) * 100) : 100;
    const failureRate = totalFinished > 0 ? Math.round((failed / totalFinished) * 100) : 0;

    let rank = 'Agent';
    if (missions.length > 10) rank = 'Elite Operative';
    else if (missions.length > 5) rank = 'Senior Agent';
    else if (missions.length > 0) rank = 'Field Agent';

    return {
      missionsTotal: missions.length,
      successPercent: successRate,
      failurePercent: failureRate,
      activeStatus: 'Active',
      rank: rank,
      joinDate: 'Jan 2024'
    };
  }

  async ngOnInit() {
    await this.loadMyMissions();
  }

  async loadMyMissions() {
    this.isLoadingMissions.set(true);
    try {
      const missions = await this._missionService.getMyMissions();
      this.recentMissions.set(missions);
    } catch (error) {
      console.error('Failed to load missions', error);
    } finally {
      this.isLoadingMissions.set(false);
    }
  }

  // ฟังก์ชันสำหรับสลับโหมดการแก้ไขชื่อ
  toggleEditName(): void {
    if (!this.isEditingName()) {
      // เมื่อเริ่มแก้ไข ให้เอาชื่อปัจจุบันไปใส่ในช่อง Input สำรอง (tempName)
      this.tempName.set(this.displayName());
      this.isEditingName.set(true); // เปิดโหมดแก้ไข
    } else {
      this.isEditingName.set(false); // ปิดโหมดแก้ไข
    }
  }

  // ฟังก์ชันสำหรับบันทึกชื่อใหม่
  async saveName(): Promise<void> {
    const newName = this.tempName().trim();

    // ถ้าชื่อว่าง หรือเป็นชื่อเดิม ไม่ต้องส่งไปที่ Server
    if (!newName || newName === this.displayName()) {
      this.isEditingName.set(false);
      return;
    }

    try {
      // เรียกใช้ Service เพื่ออัปเดตชื่อในฐานข้อมูล
      const error = await this._userService.updateDisplayName(newName);

      if (error) {
        this._snackBar.open(error, 'Close', { duration: 5000, panelClass: 'error-snackbar' });
      } else {
        // แสดงข้อความสำเร็จและปิดโหมดแก้ไข
        this._snackBar.open('เปลี่ยนชื่อสำเร็จแล้ว!', 'Success', { duration: 3000 });
        this.isEditingName.set(false);
      }
    } catch (e) {
      console.error('ไม่สามารถอัปเดตชื่อได้:', e);
    }
  }

  logout(): void {
    this._passport.destroy();
    this._router.navigate(['/']);
  }

  openDialog(): void {
    const ref = this._dialog.open(UploadPhoto, {
      data: { name: this.displayName() },
      width: '450px',
      panelClass: 'premium-dialog'
    });
    ref.afterClosed().subscribe(async (file) => {
      if (file) {
        await this._userService.uploadAvatarImg(file);
      }
    });
  }

  editProfile(): void {
    this.toggleEditName();
  }
}


