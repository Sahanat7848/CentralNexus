import { Component, Inject, OnInit, signal, effect, ElementRef, ViewChild, inject } from '@angular/core';
import { CommonModule } from '@angular/common';
import { FormsModule } from '@angular/forms';
import { MAT_DIALOG_DATA, MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatIconModule } from '@angular/material/icon';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { MessageService } from '../../_services/message-service';
import { PassportService } from '../../_services/passport-service';
import { Message } from '../../_models/message';
import { BrawlerSummary } from '../../_models/brawler';

@Component({
    selector: 'app-chat',
    standalone: true,
    imports: [
        CommonModule,
        FormsModule,
        MatDialogModule,
        MatButtonModule,
        MatIconModule,
        MatFormFieldModule,
        MatInputModule
    ],
    templateUrl: './chat.html',
    styleUrl: './chat.scss'
})
export class ChatDialog implements OnInit {
    private _messageService = inject(MessageService);
    private _passportService = inject(PassportService);

    messages = signal<Message[]>([]);
    newMessage = '';
    currentUserId = signal<number | null>(null);
    todaysDate = new Date();

    @ViewChild('scrollMe') private myScrollContainer!: ElementRef;

    constructor(
        public dialogRef: MatDialogRef<ChatDialog>,
        @Inject(MAT_DIALOG_DATA) public data: { friend: BrawlerSummary }
    ) {
        this.currentUserId.set(this._passportService.data()?.brawler_id ?? null);

        // Polling for new messages every 3 seconds for simplicity
        setInterval(() => {
            this.loadMessages();
        }, 3000);
    }

    ngOnInit() {
        this.loadMessages();
    }

    // ฟังก์ชันสำหรับโหลดข้อความแชท
    async loadMessages() {
        try {
            // เรียกใช้ Service เพื่อดึงประวัติการคุยกับเพื่อนคนนี้
            const msgs = await this._messageService.get_conversation(this.data.friend.id);

            // ตรวจสอบว่ามีข้อความใหม่หรือไม่ (เปรียบเทียบจำนวนข้อความ)
            // เพื่อหลีกเลี่ยงการกระตุกของหน้าจอหากไม่มีข้อมูลเปลี่ยนแปลง
            if (msgs.length !== this.messages().length) {
                this.messages.set(msgs); // อัปเดตข้อความใหม่
                // หน่วงเวลาเล็กน้อยเพื่อให้ UI เรนเดอร์เสร็จก่อนเลื่อนลงล่างสุด
                setTimeout(() => this.scrollToBottom(), 100);
            }
        } catch (error) {
            console.error('ไม่สามารถโหลดข้อมูลประวัติการแชทได้:', error);
        }
    }

    // ฟังก์ชันสำหรับส่งข้อความ
    async sendMessage() {
        const content = this.newMessage.trim();
        if (!content) return; // ถ้าไม่มีข้อความ ไม่ต้องทำอะไร

        try {
            // ส่งข้อความไปยัง Server
            await this._messageService.send_message(this.data.friend.id, content);

            this.newMessage = ''; // เคลียร์ช่อง Input
            await this.loadMessages(); // โหลดข้อความใหม่ทันทีเพื่อให้เห็นข้อความที่เราเพิ่งส่ง
        } catch (error) {
            console.error('การส่งข้อความล้มเหลว:', error);
        }
    }

    scrollToBottom(): void {
        try {
            this.myScrollContainer.nativeElement.scrollTop = this.myScrollContainer.nativeElement.scrollHeight;
        } catch (err) { }
    }

    close() {
        this.dialogRef.close();
    }
}
