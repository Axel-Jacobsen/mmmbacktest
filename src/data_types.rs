use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::fmt::Display;
use std::hash::Hash;

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum TimePeriod {
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "monthly")]
    Monthly,
    #[serde(rename = "allTime")]
    AllTime,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum MarketOutcome {
    // maybe not so useful, because MarketOutcome can be YES, NO,
    // and 0..\d for some reason
    #[serde(rename = "YES")]
    Yes,
    #[serde(rename = "NO")]
    No,
    #[serde(untagged)]
    Other(String),
}

impl fmt::Display for MarketOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarketOutcome::Yes => write!(f, "YES"),
            MarketOutcome::No => write!(f, "NO"),
            MarketOutcome::Other(s) => write!(f, "{}", s),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum MarketMechanism {
    #[serde(rename = "cpmm-1")]
    Cpmm,
    #[serde(rename = "cpmm-2")]
    Cpmm2,
    #[serde(rename = "cpmm-multi-1")]
    CpmmMulti,
    #[serde(rename = "dpm-2")]
    Dpm,
    #[serde(rename = "none")]
    None,
    #[serde(rename = "qf")]
    Qf,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, Eq, PartialEq)]
pub enum MarketOutcomeType {
    #[serde(rename = "BINARY")]
    Binary,
    #[serde(rename = "FREE_RESPONSE")]
    FreeResponse,
    #[serde(rename = "MULTIPLE_CHOICE")]
    MultipleChoice,
    #[serde(rename = "NUMERIC")]
    Numeric,
    #[serde(rename = "PSEUDO_NUMERIC")]
    PseudoNumeric,
    #[serde(rename = "STONK")]
    Stonk,
    #[serde(rename = "POLL")]
    Poll,
    #[serde(rename = "BOUNTIED_QUESTION")]
    BountiedQuestion,
    #[serde(rename = "QUADRATIC_FUNDING")]
    QuadraticFunding,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    /// from <https://docs.manifold.markets/api#get-v0users>
    pub id: String,

    #[serde(rename = "createdTime")]
    pub created_time: u64,

    pub name: String,
    pub username: String,

    pub url: Option<String>,

    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,

    pub bio: Option<String>,

    #[serde(rename = "bannerUrl")]
    pub banner_url: Option<String>,

    pub website: Option<String>,

    #[serde(rename = "twitterHandle")]
    pub twitter_handle: Option<String>,

    #[serde(rename = "discordHandle")]
    pub discord_handle: Option<String>,

    #[serde(rename = "isBot")]
    pub is_bot: Option<bool>,

    /// is in manifold team
    #[serde(rename = "isAdmin")]
    pub is_admin: Option<bool>,

    /// is trustworthy
    #[serde(rename = "isTrustworthy")]
    pub is_trustworthy: Option<bool>,

    #[serde(rename = "isBannedFromPosting")]
    pub is_banned_from_posting: Option<bool>,

    #[serde(rename = "userDeleted")]
    pub user_deleted: Option<bool>,

    pub balance: f64,

    #[serde(rename = "totalDeposits")]
    pub total_deposits: f64,

    #[serde(rename = "lastBetTime")]
    pub last_bet_time: Option<u64>,

    #[serde(rename = "currentBettingStreak")]
    pub current_betting_streak: Option<u64>, // guessing here

    #[serde(rename = "profitCached")]
    pub profit_cached: HashMap<TimePeriod, f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LiteMarket {
    /// from <https://docs.manifold.markets/api#get-v0markets>

    /// Unique identifer for this market
    pub id: String,

    /// The id of the creator
    #[serde(rename = "creatorId")]
    pub creator_id: String,

    /// Attributes about the creator
    #[serde(rename = "creatorUsername")]
    pub creator_username: String,

    /// The name of the creator
    #[serde(rename = "creatorName")]
    pub creator_name: String,

    #[serde(rename = "creatorAvatarUrl")]
    pub creator_avatar_url: Option<String>,

    /// Market attributes. All times are in milliseconds since epoch

    /// Min of creator's chosen date, and resolutionTime
    /// bug in the API (I think?) lets close_time be < 0
    /// see https://manifold.markets/PlasmaBallin/will-the-trinity-test-ignite-the-at
    /// Just leave as Option<i64>
    #[serde(rename = "closeTime")]
    pub close_time: Option<i64>,

    /// milliseconds since epoch
    #[serde(rename = "createdTime")]
    pub created_time: u64,

    /// The question!
    pub question: String,

    /// Note: This url always points to <https://manifold.markets>, regardless of what instance the api is running on.
    /// This url includes the creator's username, but this doesn't need to be correct when constructing valid URLs.
    ///   i.e. <https://manifold.markets/Austin/test-market> is the same as <https://manifold.markets/foo/test-market>
    pub url: String,

    /// BINARY, FREE_RESPONSE, MULTIPLE_CHOICE, NUMERIC, or PSEUDO_NUMERIC
    #[serde(rename = "outcomeType")]
    pub outcome_type: MarketOutcomeType,

    /// dpm-2 or cpmm-1 or cpmm-multi-1
    pub mechanism: MarketMechanism,

    /// current probability of the market
    pub probability: Option<f64>,

    /// For CPMM markets, the number of shares in the liquidity pool. For DPM markets,
    /// the amount of mana invested in each answer.
    // pool: Option<HashMap<MarketOutcome, f64>>,
    // ^^^^ MarketOutcome can be YES, NO, and 0..\d
    // Therefore we just do String, and we'll have to deal w/ decoding YES / NO at runtime :(
    pub pool: Option<HashMap<String, f64>>,

    /// CPMM markets only, probability constant in y^p * n^(1-p) = k
    pub p: Option<f64>,

    /// CPMM markets only, the amount of mana deposited into the liquidity pool
    #[serde(rename = "total_liquidity")]
    pub total_liquidity: Option<f64>,

    /// PSEUDO_NUMERIC markets only, the current market value, which is mapped from
    /// probability using min, max, and isLogScale.
    pub value: Option<f64>,

    /// PSEUDO_NUMERIC markets only, the minimum resolvable value
    pub min: Option<f64>,

    /// PSEUDO_NUMERIC markets only, the maximum resolvable value
    pub max: Option<f64>,

    /// PSEUDO_NUMERIC markets only, if true `number = (max - min + 1)^probability + minstart - 1`,
    /// otherwise `number = min + (max - min) * probability`
    #[serde(rename = "isLogScale")]
    pub is_log_scale: Option<bool>,

    pub volume: f64,

    #[serde(rename = "volume24Hours")]
    pub volume_24_hours: f64,

    #[serde(rename = "isResolved")]
    pub is_resolved: bool,

    #[serde(rename = "resolutionTime")]
    pub resolution_time: Option<u64>,

    pub resolution: Option<String>,

    /// Used for BINARY markets resolved to MKT
    #[serde(rename = "resolutionProbability")]
    pub resolution_probability: Option<f64>,

    #[serde(rename = "lastUpdatedTime")]
    pub last_updated_time: Option<u64>,

    #[serde(rename = "lastBetTime")]
    pub last_bet_time: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    /// Guessing on this one
    pub id: String,

    #[serde(rename = "createdTime")]
    created_time: u64,

    #[serde(rename = "avatarURL")]
    avatar_url: Option<String>,

    username: Option<String>,
    number: Option<u32>,
    name: Option<String>,

    #[serde(rename = "contractId")]
    contract_id: String,

    pub text: String,

    #[serde(rename = "userId")]
    user_id: String,
    pub probability: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JSONContent {
    // Not dealing w/ this for now
    // I don't even think it's useful
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullMarket {
    #[serde(flatten)]
    pub lite_market: LiteMarket,

    /// dpm-2 markets only
    pub answers: Option<Vec<Answer>>,

    /// Rich text content. See https://tiptap.dev/guide/output#option-1-json
    #[serde(skip_deserializing)]
    description: Option<JSONContent>,

    /// string description without formatting, images, or embeds
    #[serde(rename = "textDescription")]
    text_description: Option<String>,

    /// groups which the market is a part of
    #[serde(rename = "groupSlugs")]
    group_slugs: Option<Vec<String>>,
}

/// A single position in a market
#[derive(Serialize, Deserialize, Debug)]
pub struct ContractMetric {
    /// From Here https://docs.manifold.markets/api#get-v0marketmarketidpositions

    /// The contract ID
    #[serde(rename = "contractId")]
    contract_id: String,

    /// Includes day, week, month. Can be undefined.
    from: Option<HashMap<String, PeriodMetric>>,

    /// Indicates if there are no shares
    #[serde(rename = "hasNoShares")]
    has_no_shares: bool,

    /// Indicates if there are shares
    #[serde(rename = "hasShares")]
    has_shares: bool,

    /// Indicates if there are yes shares
    #[serde(rename = "hasYesShares")]
    has_yes_shares: bool,

    /// Invested amount
    invested: f64,

    /// Loan amount
    loan: f64,

    /// Maximum shares outcome, can be null
    #[serde(rename = "maxSharesOutcome")]
    max_shares_outcome: Option<String>,

    /// Payout amount
    payout: f64,

    /// Profit amount
    profit: f64,

    /// Profit percentage
    #[serde(rename = "profitPercent")]
    profit_percent: f64,

    /// Total shares
    #[serde(rename = "totalShares")]
    total_shares: HashMap<MarketOutcome, f64>,

    /// User ID
    #[serde(rename = "userId")]
    user_id: String,

    /// User name
    #[serde(rename = "userName")]
    user_name: String,

    /// User avatar URL
    #[serde(rename = "userAvatarUrl")]
    user_avatar_url: String,

    /// Last bet time
    #[serde(rename = "lastBetTime")]
    last_bet_time: u64,
}

/// Metrics for a specific period
#[derive(Serialize, Deserialize, Debug)]
pub struct PeriodMetric {
    /// Profit amount
    profit: f64,
    /// Profit percentage
    #[serde(rename = "profitPercent")]
    profit_percent: f64,
    /// Invested amount
    invested: f64,
    /// Previous value
    #[serde(rename = "prevValue")]
    prev_value: f64,
    /// Current value
    value: f64,
}

/// Represents a bet
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bet {
    /// From https://github.com/manifoldmarkets/manifold/blob/main/common/src/bet.ts
    pub id: String,

    #[serde(rename = "userId")]
    pub user_id: String,

    // denormalized for bet lists (whatever that means)
    #[serde(rename = "userAvatarUrl", skip_serializing_if = "Option::is_none")]
    pub user_avatar_url: Option<String>,

    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    #[serde(rename = "userUsername", skip_serializing_if = "Option::is_none")]
    pub user_username: Option<String>,

    #[serde(rename = "contractId")]
    pub contract_id: String,

    /// For multi-binary contracts. Optional.
    #[serde(rename = "answerId", skip_serializing_if = "Option::is_none")]
    pub answer_id: Option<String>,

    #[serde(rename = "createdTime")]
    pub created_time: u64,

    /// Bet size; negative if SELL bet
    pub amount: f64,

    /// Optional loan amount
    #[serde(rename = "loanAmount", skip_serializing_if = "Option::is_none")]
    pub loan_amount: Option<f64>,

    pub outcome: String,

    /// Dynamic parimutuel pool weight or fixed; negative if SELL bet
    pub shares: f64,

    /// Deprecated: Gain shares in multiple outcomes. Part of cpmm-2 multiple choice.
    #[deprecated(note = "Use alternative field")]
    #[serde(rename = "sharesByOutcome", skip_serializing_if = "Option::is_none")]
    pub shares_by_outcome: Option<HashMap<String, f64>>,

    #[serde(rename = "probBefore")]
    pub prob_before: f64,

    #[serde(rename = "probAfter")]
    pub prob_after: f64,

    pub fees: Option<Fees>,

    /// True if bet was placed via API. Optional.
    #[serde(rename = "isApi", skip_serializing_if = "Option::is_none")]
    pub is_api: Option<bool>,

    #[serde(rename = "isAnte")]
    pub is_ante: bool,

    #[serde(rename = "isRedemption")]
    pub is_redemption: bool,

    #[serde(rename = "isChallenge")]
    pub is_challenge: bool,

    pub visibility: Visibility,

    /// Optional challenge slug
    #[serde(rename = "challengeSlug", skip_serializing_if = "Option::is_none")]
    pub challenge_slug: Option<String>,

    /// Optional reply to comment ID
    #[serde(rename = "replyToCommentId", skip_serializing_if = "Option::is_none")]
    pub reply_to_comment_id: Option<String>,

    #[serde(flatten)]
    pub limit_props: Option<LimitProps>,
}

impl Display for Bet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let buysell = if self.amount < 0.0 { "SELL" } else { "BUY" };
        write!(
            f,
            "contract id: {} | answer id: {} | bet: {:.2} {} {}",
            self.contract_id,
            self.answer_id.clone().unwrap_or_default(),
            self.amount.abs(),
            buysell,
            self.outcome
        )
    }
}

/// NumericBet extends Bet with additional fields
#[derive(Serialize, Deserialize, Debug)]
pub struct NumericBet {
    #[serde(flatten)]
    bet: Bet,
    value: f64,
    #[serde(rename = "allOutcomeShares")]
    all_outcome_shares: HashMap<String, f64>,
    #[serde(rename = "allBetAmounts")]
    all_bet_amounts: HashMap<String, f64>,
}

/// LimitBet is a Bet with LimitProps flattened into it
#[derive(Serialize, Deserialize, Debug)]
pub struct LimitBet {
    #[serde(flatten)]
    bet: Bet,
    #[serde(flatten)]
    limit_props: LimitProps,
}

/// Properties specific to a limit bet
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LimitProps {
    /// Amount of mana in the order
    #[serde(rename = "orderAmount")]
    order_amount: f64,
    /// [0, 1]. Bet to this probability.
    #[serde(rename = "limitProb")]
    limit_prob: f64,
    /// Whether all of the bet amount has been filled.
    #[serde(rename = "isFilled")]
    is_filled: bool,
    /// Whether to prevent any further fills.
    #[serde(rename = "isCancelled")]
    is_cancelled: bool,
    /// A record of each transaction that partially (or fully) fills the order amount.
    fills: Vec<Fill>,
    /// ms since epoch. Optional.
    #[serde(rename = "expiresAt", skip_serializing_if = "Option::is_none")]
    expires_at: Option<u64>,
}

/// Represents a fill in a bet
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fill {
    /// Timestamp of the fill
    timestamp: u64,
    /// The id the bet matched against, or null if the bet was matched by the pool.
    #[serde(rename = "matchedBetId")]
    matched_bet_id: Option<String>,
    /// Amount involved in the fill
    amount: f64,
    /// Shares involved in the fill
    shares: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Fees {
    /// Fee for the creator
    #[serde(rename = "creatorFee")]
    creator_fee: f64,

    /// Fee for the platform
    #[serde(rename = "platformFee")]
    platform_fee: f64,

    /// Fee for liquidity
    #[serde(rename = "liquidityFee")]
    liquidity_fee: f64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Unlisted,
    Private,
}
