// Copyright (C) 2026 UberMetroid
//
// This file is part of Rustle.

use crate::i18n::Translations;

pub fn translations() -> Translations {
    Translations {
        game_title: "Rustle",
        win_messages: &["太棒了！", "出色", "做得好！"],
        game_copied: "游戏结果已复制到剪贴板",
        not_enough_letters: "字数不够",
        word_not_found: "未找到单词",
        hard_mode_alert: "困难模式只能在开始时启用！",
        enter: "确定",
        delete: "退格",
        statistics: "统计数据",
        guess_distribution: "猜测分布",
        new_word: "下个单词",
        share: "分享",
        share_failure: "无法分享结果。此功能仅在安全上下文 (HTTPS) 中可用。",
        transfer: "传输",
        transfer_desc: "点击此处将您的统计数据传输到新设备。",
        total_tries: "总尝试次数",
        success_rate: "胜率",
        current_streak: "当前连续",
        best_streak: "最佳连续",
        discourage_browser:
            "您正在使用内置浏览器，分享或保存结果时可能会遇到问题。我们建议您使用默认浏览器。",
        datepicker_title: "选择过去的日期",
        datepicker_choose: "选择",
        logout: "登出",
    }
}
