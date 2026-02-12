import { HttpInterceptorFn } from '@angular/common/http';
import { PassportService } from '../_services/passport-service';
import { inject } from '@angular/core';

export const jwtInterceptor: HttpInterceptorFn = (req, next) => {
  const passport = inject(PassportService)
  const token = passport.data()?.access_token
  if (token) {
    const Authorization = `Bearer ${token}`
    req = req.clone({
      setHeaders: {
        Authorization
      }
    })
  }
  return next(req);
};
