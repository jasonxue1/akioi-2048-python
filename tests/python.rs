use anyhow::{Context, Result};
use once_cell::sync::Lazy;
use std::process::Command;

/// ───────────────────────────────────────────────
/// ① 只执行一次：检查 uv ➜  uv sync --verbose
/// ───────────────────────────────────────────────
static UV_SYNC: Lazy<Result<()>> = Lazy::new(|| {
    // 1️⃣ 确认 uv 在 PATH
    which::which("uv").context("`uv` not found; install with `pip install uv`")?;

    // 2️⃣ 同步依赖 / 创建 .venv
    let ok = Command::new("uv")
        .args(["sync", "--verbose"])
        .status()
        .context("failed to run `uv sync --verbose`")?
        .success();
    anyhow::ensure!(ok, "`uv sync --verbose` failed");

    Ok(())
});

/// 帮助函数：确保全局 UV_SYNC 已成功完成
fn ensure_uv_sync() -> Result<()> {
    UV_SYNC
        .as_ref()
        .map_err(|e| anyhow::anyhow!("uv sync failed: {e}"))
        .map(|_| ())
}

/// ───────────────────────────────────────────────
/// ② util：在已同步的 venv 里运行脚本
/// ───────────────────────────────────────────────
fn run_py(script: &str) -> Result<()> {
    ensure_uv_sync()?; // 保证同步已完成

    let ok = Command::new("uv")
        .args(["run", script])
        .status()
        .with_context(|| format!("failed to run `{script}`"))?
        .success();
    anyhow::ensure!(ok, "`{script}` reported test failures");
    Ok(())
}

/// -------------------------------------------------------
/// ③ 具体测试条目（cargo test 会分别显示 3 行）
/// -------------------------------------------------------
#[test]
fn uv_sync_package() -> Result<()> {
    ensure_uv_sync()
}

#[test]
fn test_init() -> Result<()> {
    run_py("tests/test_init.py")
}
