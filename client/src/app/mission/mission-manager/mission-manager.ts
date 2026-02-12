import { Component, OnInit, inject } from '@angular/core';
import { MissionService } from '../../_services/mission-service';
import { MatDialog } from '@angular/material/dialog';
import { Mission } from '../../_models/mission';
import { NewMission } from '../../_dialog/new-mission/new-mission';
import { AddMission } from '../../_models/add-mission';
import { DatePipe } from '@angular/common';
import { AsyncPipe } from '@angular/common';
import { MatTableDataSource, MatTableModule } from '@angular/material/table';
import { MatPaginator, MatPaginatorModule } from '@angular/material/paginator';
import { MatSort, MatSortModule } from '@angular/material/sort';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { CommonModule } from '@angular/common';
import { ViewChild } from '@angular/core';

@Component({
  selector: 'app-mission-manager',
  standalone: true,
  imports: [
    CommonModule,
    MatIconModule,
    DatePipe,
    MatTableModule,
    MatPaginatorModule,
    MatSortModule,
    MatProgressSpinnerModule,
    MatButtonModule
  ],
  templateUrl: './mission-manager.html',
  styleUrl: './mission-manager.scss',
})
export class MissionManager implements OnInit {
  private _missionService = inject(MissionService);
  private _dialog = inject(MatDialog);

  dataSource = new MatTableDataSource<Mission>([]);
  displayedColumns: string[] = ['name', 'description', 'status', 'crew_count', 'created_at'];
  isLoadingResults = true;
  isRateLimitReached = false;
  resultsLength = 0;
  totalCrew = 0;
  successRate = 100;

  @ViewChild(MatPaginator) paginator!: MatPaginator;
  @ViewChild(MatSort) sort!: MatSort;

  ngOnInit() {
    this.loadMyMission();
  }

  private async loadMyMission() {
    this.isLoadingResults = true;
    try {
      const missions = await this._missionService.getMyMissions();
      this.dataSource.data = missions;
      this.dataSource.paginator = this.paginator;
      this.dataSource.sort = this.sort;
      this.resultsLength = missions.length;
      this.calculateStats();
    } catch (e) {
      console.error(e);
      this.isRateLimitReached = true;
    } finally {
      this.isLoadingResults = false;
    }
  }

  openDialog() {
    const ref = this._dialog.open(NewMission);
    ref.afterClosed().subscribe(async (mission_data: AddMission) => {
      if (!mission_data) return;
      const id = await this._missionService.add(mission_data);
      const now = new Date();
      const newMission: Mission = {
        id,
        name: mission_data.name,
        description: mission_data.description,
        status: 'Open',
        chief_id: 0,
        chief_display_name: 'You',
        crew_count: 0,
        created_at: now,
        updated_at: now,
      };

      const currentMissions = this.dataSource.data;
      this.dataSource.data = [...currentMissions, newMission];
      this.resultsLength = this.dataSource.data.length;
      this.calculateStats();
    });
  }

  private calculateStats() {
    const missions = this.dataSource.data;
    this.totalCrew = missions.reduce((acc, m) => acc + (m.crew_count || 0), 0);
    const completed = missions.filter(m => m.status === 'Completed').length;
    this.successRate = missions.length > 0 ? Math.round((completed / missions.length) * 100) : 100;
  }
}
