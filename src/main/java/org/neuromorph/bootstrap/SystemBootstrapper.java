package org.neuromorph.bootstrap;

import org.neuromorph.sovereignty.SovereignNeuromorphContract;

import java.io.IOException;
import java.io.UncheckedIOException;
import java.nio.charset.StandardCharsets;
import java.nio.file.*;
import java.time.Instant;
import java.time.ZoneOffset;
import java.time.format.DateTimeFormatter;
import java.util.*;

/**
 * SystemBootstrapper
 *
 * Refactored from the original C# example to Java, preserving:
 * - Directory layout: DailyImplementations/yyyy-MM-dd/{Configs,Modules,Resources,Logs,Backups}
 * - Asset creation if missing.
 * - Logging to both console and file.
 *
 * Extended with neuromorphic-sovereignty invariants:
 * - No operation proceeds without explicit consent from the SovereignNeuromorphContract.
 * - No procedure may implement downgrade/rollback semantics.
 * - All activity is logged in an auditable, append-only manner.
 */
public final class SystemBootstrapper {

    private static final String[] ASSET_DIRECTORIES = {
            "Configs",
            "Modules",
            "Resources",
            "Logs",
            "Backups"
    };

    private static final DateTimeFormatter DATE_FORMAT =
            DateTimeFormatter.ofPattern("yyyy-MM-dd").withZone(ZoneOffset.UTC);

    private static final DateTimeFormatter TIMESTAMP_FORMAT =
            DateTimeFormatter.ISO_INSTANT;

    private final SovereignNeuromorphContract contract;
    private final Path baseDailyImplementationDir;
    private final Path logFilePath;

    public SystemBootstrapper(SovereignNeuromorphContract contract, Path baseRoot) {
        this.contract = Objects.requireNonNull(contract, "contract must not be null");
        Objects.requireNonNull(baseRoot, "baseRoot must not be null");

        String today = DATE_FORMAT.format(Instant.now());
        this.baseDailyImplementationDir = baseRoot
                .resolve("DailyImplementations")
                .resolve(today)
                .toAbsolutePath()
                .normalize();

        this.logFilePath = baseDailyImplementationDir.resolve("SystemBootstrapper.log");
    }

    public void run() {
        enforceSovereigntyPreconditions();

        log("=== System Bootstrapper Started ===");
        ensureDirectories();
        buildOrLocateAssets();
        expandAssets();
        log("=== System Bootstrapper Completed ===");
    }

    private void enforceSovereigntyPreconditions() {
        if (!contract.hasExplicitConsent()) {
            throw new IllegalStateException(
                    "SNC violation: explicit consent is required before bootstrap operations.");
        }
        if (!contract.hasSovereignAbortControl()) {
            throw new IllegalStateException(
                    "SNC violation: sovereign abort control is mandatory for neuromorphic participants.");
        }
        if (!contract.isDisciplinePersonalizedAndNonCoercive()) {
            throw new IllegalStateException(
                    "SNC violation: discipline must be personalized, non-coercive, and tied to learning objectives.");
        }
        if (!contract.forbidsDowngradeOrRollback()) {
            throw new IllegalStateException(
                    "SNC violation: downgrade/rollback of neuromorphic capabilities is prohibited.");
        }
    }

    private void ensureDirectories() {
        try {
            Files.createDirectories(baseDailyImplementationDir);
            for (String dir : ASSET_DIRECTORIES) {
                Files.createDirectories(baseDailyImplementationDir.resolve(dir));
            }
            log("Directories ensured at: " + baseDailyImplementationDir);
        } catch (IOException e) {
            throw new UncheckedIOException("Failed to ensure directories.", e);
        }
    }

    private void buildOrLocateAssets() {
        List<String> missingAssets = getMissingAssets();
        for (String asset : missingAssets) {
            buildAsset(asset);
            log("Asset built: " + asset);
        }
        log(missingAssets.size() + " missing assets built or located.");
    }

    private List<String> getMissingAssets() {
        List<String> requiredAssets = Arrays.asList(
                "Configs/SystemConfig.json",
                "Modules/AuthModule.dll",
                "Resources/Localization.en-US.resx",
                "Logs/Startup.log",
                "Backups/Initial.bak"
        );

        List<String> missing = new ArrayList<>();
        for (String asset : requiredAssets) {
            Path fullPath = baseDailyImplementationDir.resolve(asset);
            if (!Files.exists(fullPath)) {
                missing.add(asset);
            }
        }
        log("Missing assets: " + (missing.isEmpty() ? "(none)" : String.join(", ", missing)));
        return missing;
    }

    private void buildAsset(String asset) {
        Path fullPath = baseDailyImplementationDir.resolve(asset);
        Path parent = fullPath.getParent();
        try {
            if (parent != null) {
                Files.createDirectories(parent);
            }
            String content = "// Auto-generated asset: "
                    + asset
                    + System.lineSeparator()
                    + "// Generated on "
                    + TIMESTAMP_FORMAT.format(Instant.now())
                    + System.lineSeparator();

            Files.write(
                    fullPath,
                    content.getBytes(StandardCharsets.UTF_8),
                    StandardOpenOption.CREATE,
                    StandardOpenOption.TRUNCATE_EXISTING,
                    StandardOpenOption.WRITE
            );
        } catch (IOException e) {
            throw new UncheckedIOException("Failed to build asset: " + asset, e);
        }
    }

    private void expandAssets() {
        String newAssetName = "Resources/AutoAsset_" + UUID.randomUUID() + ".txt";
        buildAsset(newAssetName);
        log("Expanded with new asset: " + newAssetName);
    }

    private void log(String message) {
        String entry = TIMESTAMP_FORMAT.format(Instant.now()) + " | " + message;
        System.out.println(entry);

        try {
            Path parent = logFilePath.getParent();
            if (parent != null) {
                Files.createDirectories(parent);
            }
            Files.write(
                    logFilePath,
                    (entry + System.lineSeparator()).getBytes(StandardCharsets.UTF_8),
                    StandardOpenOption.CREATE,
                    StandardOpenOption.APPEND,
                    StandardOpenOption.WRITE
            );
        } catch (IOException e) {
            throw new UncheckedIOException("Failed to write log entry.", e);
        }
    }
}
