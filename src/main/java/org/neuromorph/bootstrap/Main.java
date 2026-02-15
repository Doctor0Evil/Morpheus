package org.neuromorph.bootstrap;

import org.neuromorph.sovereignty.DefaultSovereignNeuromorphContract;
import org.neuromorph.sovereignty.SovereignNeuromorphContract;

import java.nio.file.Paths;

/**
 * Example entry point, demonstrating how the bootstrapper is invoked
 * under a sovereign neuromorphic contract.
 */
public final class Main {

    public static void main(String[] args) {
        SovereignNeuromorphContract contract =
                new DefaultSovereignNeuromorphContract(
                        true,   // explicitConsent
                        true,   // sovereignAbortControl
                        true    // personalizedNonCoerciveDiscipline
                );

        SystemBootstrapper bootstrapper =
                new SystemBootstrapper(contract, Paths.get("."));

        bootstrapper.run();
    }
}
