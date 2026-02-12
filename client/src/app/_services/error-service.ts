import { inject, Injectable } from '@angular/core';
import { NavigationExtras, Router } from '@angular/router';
import { MatSnackBar } from '@angular/material/snack-bar';
import { MatSnackBarConfig } from '@angular/material/snack-bar';
import { Observable, throwError } from 'rxjs';
@Injectable({
  providedIn: 'root',
})
export class ErrorService {
  private _router = inject(Router)
  private _snackbar = inject(MatSnackBar)
  private _snackBarConfig: MatSnackBarConfig = {
    horizontalPosition: 'right',
    verticalPosition: 'top',
  }

  handleError = (error: any): Observable<never> => {
    if (error) {
      console.error('Error Intercepted:', error);
      const errorMessage = error.error?.message || error.error || error.message || 'Something went wrong';

      switch (error.status) {
        case 400:
          this._snackbar.open(`Bad Request: ${errorMessage}`, 'OK', this._snackBarConfig);
          break;
        case 401:
          this._snackbar.open('Session expired or unauthorized. Please login again.', 'OK', this._snackBarConfig);
          break;
        case 403:
          this._snackbar.open('You do not have permission to perform this action.', 'OK', this._snackBarConfig);
          break;
        case 404:
          this._router.navigate(['/not-found']);
          break;
        case 413:
          this._snackbar.open('File is too large! Maximum limit is 10MB.', 'OK', this._snackBarConfig);
          break;
        case 500:
          const navigationExtras: NavigationExtras = { state: { error: error.error } };
          this._router.navigate(['/server-error'], navigationExtras);
          break;
        case 0:
          this._snackbar.open('Cannot connect to server. Please check your internet connection.', 'OK', this._snackBarConfig);
          break;
        default:
          this._snackbar.open(`Error ${error.status}: ${errorMessage}`, 'OK', this._snackBarConfig);
          break;
      }
    }
    return throwError(() => error);
  };
}
