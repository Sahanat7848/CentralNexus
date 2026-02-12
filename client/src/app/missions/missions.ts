import { Component, inject, ViewChild, AfterViewInit, computed, Signal, OnInit } from '@angular/core';
import { PassportService } from '../_services/passport-service';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MissionService } from '../_services/mission-service';
import { Mission } from '../_models/mission';
import { MissionFilter } from '../_models/mission-filter';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { MatPaginator, MatPaginatorModule } from '@angular/material/paginator';
import { MatSort, MatSortModule } from '@angular/material/sort';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MatSelectModule } from '@angular/material/select';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatCardModule } from '@angular/material/card';
import { ActivatedRoute, Router } from '@angular/router';

@Component({
  selector: 'app-missions',
  standalone: true,
  imports: [
    CommonModule,
    FormsModule,
    MatTableModule,
    MatPaginatorModule,
    MatSortModule,
    MatProgressSpinnerModule,
    MatFormFieldModule,
    MatInputModule,
    MatSelectModule,
    MatButtonModule,
    MatIconModule,
    MatCardModule
  ],
  templateUrl: './missions.html',
  styleUrl: './missions.scss',
})
export class Missions implements AfterViewInit, OnInit {
  private _missionService = inject(MissionService);
  private _passportService = inject(PassportService);
  private _route = inject(ActivatedRoute);
  private _router = inject(Router);

  isSignin: Signal<boolean>;

  filter: MissionFilter = {};
  dataSource = new MatTableDataSource<Mission>([]);

  baseColumns = ['name', 'description', 'chief_display_name', 'crew_count', 'status', 'created_at', 'updated_at'];
  displayedColumns = computed(() => {
    return this.isSignin() ? [...this.baseColumns, 'actions'] : this.baseColumns;
  });

  isLoading = false;

  @ViewChild(MatPaginator) paginator!: MatPaginator;
  @ViewChild(MatSort) sort!: MatSort;

  constructor() {
    this.isSignin = computed(() => this._passportService.isSignin());
  }

  async ngOnInit() {
    this._route.queryParams.subscribe(params => {
      const brawlerId = params['brawler_id'];
      if (brawlerId) {
        this.filter.brawler_id = Number(brawlerId);
      } else {
        this.filter = this._missionService.filter;
      }
      this.onSubmit();
    });
  }

  ngAfterViewInit() {
    this.dataSource.paginator = this.paginator;
    this.dataSource.sort = this.sort;
  }

  async onSubmit() {
    this.isLoading = true;
    try {
      const results = await this._missionService.getByFilter(this.filter);

      // If signed in, enrich missions with join status
      if (this.isSignin()) {
        const myMissions = await this._missionService.getMyMissions();
        const myMissionIds = new Set(myMissions.map(m => m.id));
        const currentUser = this._passportService.data();

        results.forEach(m => {
          m.is_joined = myMissionIds.has(m.id);
          m.is_chief = m.chief_id === currentUser?.brawler_id || m.chief_display_name === currentUser?.display_name;
        });
      }

      this.dataSource.data = results;
    } catch (e) {
      console.error('Error fetching missions:', e);
    } finally {
      this.isLoading = false;
    }
  }

  clearFilter() {
    this.filter = { name: undefined, status: undefined, brawler_id: undefined };
    this._router.navigate([], { relativeTo: this._route, queryParams: {} });
    this.onSubmit();
  }

  // ฟังก์ชันสำหรับกดเข้าร่วมภารกิจ
  async joinMission(mission: Mission) {
    // ตรวจสอบก่อนว่าเราเป็นหัวหน้าหรือเข้าร่วมไปแล้วหรือไม่
    if (mission.is_chief || mission.is_joined) return;

    // --- ส่วนของ Optimistic Update (เปลี่ยนหน้าจอทันทีเพื่อให้ดูเร็ว) ---
    const prevStatus = mission.is_joined; // เก็บสถานะเก่าไว้เผื่อต้องย้อนกลับ
    const prevCount = mission.crew_count; // เก็บจำนวนคนเก่าไว้
    mission.is_joined = true;             // เปลี่ยนสถานะเป็น "เข้าร่วมแล้ว" ทันที
    mission.crew_count++;                // เพิ่มจำนวนคนขึ้น 1 คนทันที

    try {
      // ส่งคำสั่งไปยัง Server เพื่อบันทึกลงฐานข้อมูลจริง
      await this._missionService.joinMission(mission.id);
      // ดึงข้อมูลใหม่มาอัปเดตหน้าจออีกครั้งเพื่อความแม่นยำ
      this.onSubmit();
    } catch (e) {
      // หากเกิดข้อผิดพลาด ให้ดึงข้อมูลเก่ากลับมาแสดงผล
      mission.is_joined = prevStatus;
      mission.crew_count = prevCount;
      console.error('เกิดข้อผิดพลาดในการเข้าร่วมภารกิจ:', e);
    }
  }

  // ฟังก์ชันสำหรับกดออกจากภารกิจ
  async leaveMission(mission: Mission) {
    // ตรวจสอบว่าไม่ใช่หัวหน้า และต้องอยู่ในภารกิจนั้นจริงๆ
    if (mission.is_chief || !mission.is_joined) return;

    // --- ส่วนของ Optimistic Update (ลดจำนวนคนทันทีโดยไม่ต้องรอ Server) ---
    const prevStatus = mission.is_joined;
    const prevCount = mission.crew_count;
    mission.is_joined = false; // เปลี่ยนสถานะเป็น "ยังไม่ได้เข้าร่วม" ทันที
    mission.crew_count--;      // ลดจำนวนคนลง 1 คนทันที

    try {
      // ส่งคำสั่งไปยัง Server เพื่อลบชื่อเราออก
      await this._missionService.leaveMission(mission.id);
      // อัปเดตข้อมูลหน้าจอใหม่
      this.onSubmit();
    } catch (e) {
      // หากล้มเหลว ให้กลับไปใช้ค่าเดิม
      mission.is_joined = prevStatus;
      mission.crew_count = prevCount;
      console.error('เกิดข้อผิดพลาดในการออกจากภารกิจ:', e);
    }
  }
}
