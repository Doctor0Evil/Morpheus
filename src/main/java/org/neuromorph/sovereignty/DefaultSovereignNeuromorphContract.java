package org.neuromorph.sovereignty;

/**
 * Default SNC implementation: all guarantees are enforced as immutable settings.
 */
public final class DefaultSovereignNeuromorphContract implements SovereignNeuromorphContract {

    private final boolean explicitConsent;
    private final boolean sovereignAbortControl;
    private final boolean personalizedNonCoerciveDiscipline;

    public DefaultSovereignNeuromorphContract(
            boolean explicitConsent,
            boolean sovereignAbortControl,
            boolean personalizedNonCoerciveDiscipline) {

        this.explicitConsent = explicitConsent;
        this.sovereignAbortControl = sovereignAbortControl;
        this.personalizedNonCoerciveDiscipline = personalizedNonCoerciveDiscipline;
    }

    @Override
    public boolean hasExplicitConsent() {
        return explicitConsent;
    }

    @Override
    public boolean hasSovereignAbortControl() {
        return sovereignAbortControl;
    }

    @Override
    public boolean isDisciplinePersonalizedAndNonCoercive() {
        return personalizedNonCoerciveDiscipline;
    }
}
