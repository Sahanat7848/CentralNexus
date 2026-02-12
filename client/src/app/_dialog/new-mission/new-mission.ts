import { Component, inject } from '@angular/core';
import { AddMission } from '../../_models/add-mission';
import { MatDialogRef, MatDialogContent, MatDialogActions, MatDialogTitle } from '@angular/material/dialog';
import { MatButtonModule } from "@angular/material/button";
import { MatInputModule } from '@angular/material/input';
import { MatIconModule } from '@angular/material/icon';
import { MatFormFieldModule } from '@angular/material/form-field';
import { FormsModule } from '@angular/forms';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-new-mission',
  imports: [
    CommonModule,
    MatDialogTitle,
    MatDialogContent,
    MatDialogActions,
    MatButtonModule,
    MatFormFieldModule,
    MatInputModule,
    MatIconModule,
    FormsModule
  ],
  templateUrl: './new-mission.html',
  styleUrl: './new-mission.scss',
})
export class NewMission {
  addMission: AddMission = {
    name: '',
    description: '',
  }
  private readonly _dialogRef = inject(MatDialogRef<NewMission>);

  onSubmit() {
    const mission = this.clean(this.addMission)
    this._dialogRef.close(mission)
  }
  private clean(addMission: AddMission) {
    return {
      name: addMission.name.trim() || 'untitle',
      description: addMission.description.trim() || undefined,
    }
  }
}
