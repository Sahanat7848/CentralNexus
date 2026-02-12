import { Component, ChangeDetectionStrategy, inject, signal } from '@angular/core';
import {
  MatDialogTitle,
  MatDialogContent,
  MatDialogActions,
  MatDialogRef,
  MAT_DIALOG_DATA,
  MatDialogClose,
} from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { CommonModule } from '@angular/common';

@Component({
  selector: 'app-upload-photo',
  imports: [
    MatDialogTitle,
    MatDialogContent,
    MatDialogActions,
    MatButtonModule,
    MatDialogClose,
    MatIconModule,
    CommonModule,
  ],
  templateUrl: './upload-photo.html',
  styleUrl: './upload-photo.scss',
  changeDetection: ChangeDetectionStrategy.OnPush,
})
export class UploadPhoto {
  data = inject<{ name: string }>(MAT_DIALOG_DATA);
  acceptedImageTypes = ['image/jpeg', 'image/png'];
  imgFile = signal<File | undefined>(undefined);
  imgPreview = signal<string | undefined>(undefined);
  errorMessage = signal<string | undefined>(undefined);
  private readonly _dialogRef = inject(MatDialogRef<UploadPhoto>);

  onSubmit() {
    this._dialogRef.close(this.imgFile());
  }

  onImagePicked(event: Event) {
    this.imgPreview.set(undefined);
    this.errorMessage.set(undefined);
    this.imgFile.set(undefined);

    const input = event.target as HTMLInputElement;
    if (input.files && input.files.length > 0) {
      const file = input.files[0];

      // ใช้ file.type แทน file-type library
      if (this.acceptedImageTypes.includes(file.type)) {
        this.imgFile.set(file);
        const reader = new FileReader();
        reader.onload = () => {
          this.imgPreview.set(reader.result as string);
        };
        reader.readAsDataURL(file);
      } else {
        this.imgFile.set(undefined);
        this.errorMessage.set('Image file must be .jpeg, or .png');
      }
    }
  }
}