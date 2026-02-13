# FieldForge — Complete Product Specification & AI Implementation Prompts

## The #1 Universal Job Management Platform for Tradespeople

**Version:** 1.0.0
**Classification:** Internal — Product & Engineering Specification
**Standard:** Apple Principal Engineer ICT Level 7+ / Microsoft Enterprise Grade

---

## TABLE OF CONTENTS

1. [Product Vision & Market Position](#1-product-vision--market-position)
2. [Target Trades & Verticals](#2-target-trades--verticals)
3. [Core Platform Architecture](#3-core-platform-architecture)
4. [Feature Specification — Complete](#4-feature-specification--complete)
5. [AI/ML Feature Suite](#5-aiml-feature-suite)
6. [Offline-First Architecture](#6-offline-first-architecture)
7. [Payment & Financial Engine](#7-payment--financial-engine)
8. [Compliance, Licensing & Insurance](#8-compliance-licensing--insurance)
9. [Analytics & Business Intelligence](#9-analytics--business-intelligence)
10. [Growth & Viral Mechanics](#10-growth--viral-mechanics)
11. [Monetization & Pricing Tiers](#11-monetization--pricing-tiers)
12. [Technical Infrastructure](#12-technical-infrastructure)
13. [AI Implementation Prompts](#13-nuclear-ai-implementation-prompts)

---

## 1. PRODUCT VISION & MARKET POSITION

### Mission Statement

Eliminate every piece of paper, every lost text message, and every unpaid invoice from the trades industry. FieldForge is the operating system for every tradesperson — from a solo handyman to a 10-person roofing crew.

### Market Reality

- **11.4M+ trade workers** in the US alone (BLS 2025)
- **78% operate without dedicated software** — using paper, texts, WhatsApp, spreadsheets
- **Average tradesperson loses $12,000/year** to missed follow-ups, unbilled work, and inefficient routing
- **Existing solutions** (ServiceTitan $200-500/mo, Housecall Pro $65-249/mo, Jobber $49-249/mo) are enterprise-focused, bloated, and overpriced for small operators
- **TAM:** $3.2B (US field service management for SMB trades)
- **SAM:** $890M (solo operators and crews of 1-10)

### Competitive Moat

1. **Offline-first** — works in basements, attics, crawl spaces, rural areas with zero signal
2. **Trade-specific intelligence** — not generic project management; understands plumbing vs electrical vs HVAC workflows
3. **AI photo estimation** — snap a photo, get line items and pricing instantly
4. **Price point** — $19/mo solo vs $200+/mo competitors
5. **30-second onboarding** — productive in under a minute, no training required
6. **Tap-to-pay** — collect payment on the spot, not 30 days later

---

## 2. TARGET TRADES & VERTICALS

### Primary Trades (Launch)

| Trade | US Workers | Key Workflows |
|-------|-----------|---------------|
| Plumbing | 560K | Leak detection, pipe repair, fixture install, drain clearing |
| Electrical | 762K | Panel upgrades, outlet install, lighting, wiring |
| HVAC | 394K | Install, maintenance, repair, duct cleaning |
| General Contractors | 890K | Remodels, additions, new construction management |
| Painters | 310K | Interior/exterior, prep, color matching, surface repair |
| Carpenters | 725K | Framing, trim, cabinets, decks, furniture |
| Roofers | 210K | Inspection, repair, replacement, flashing |
| Landscapers | 1.1M | Mowing, hardscape, irrigation, seasonal cleanup |
| Handymen | 680K | Multi-trade small jobs, honey-do lists |
| Flooring | 185K | Tile, hardwood, LVP, carpet, subfloor repair |

### Secondary Trades (Phase 2)

| Trade | US Workers | Key Workflows |
|-------|-----------|---------------|
| Concrete/Masonry | 320K | Foundations, driveways, patios, retaining walls |
| Welders | 430K | Fabrication, repair, structural, ornamental |
| Garage Door | 45K | Install, repair, opener replacement, spring repair |
| Fencing | 95K | Wood, vinyl, chain link, gates, repair |
| Insulation | 78K | Blown-in, spray foam, batt, removal |
| Drywall | 175K | Hang, tape, mud, texture, repair |
| Window/Door | 110K | Install, replacement, storm, sliding |
| Appliance Repair | 55K | Diagnostic, parts ordering, install |
| Locksmith | 35K | Rekey, install, emergency, commercial |
| Pool/Spa | 85K | Maintenance, repair, equipment, opening/closing |
| Chimney | 22K | Sweep, inspection, liner, cap, repair |
| Pest Control | 175K | Inspection, treatment, prevention, wildlife |
| Septic | 28K | Pumping, inspection, repair, install |
| Tree Service | 145K | Trim, removal, stump grinding, emergency |
| Pressure Washing | 65K | Residential, commercial, fleet, restoration |
| Gutter | 42K | Install, repair, cleaning, guards |
| Solar | 260K | Install, maintenance, inspection, battery |
| EV Charger Install | 15K | Residential, commercial, panel upgrade |
| Fire Protection | 85K | Sprinkler, alarm, inspection, extinguisher |
| Elevator/Lift | 25K | Maintenance, repair, inspection, modernization |

### Trade-Specific Configurations

Each trade gets a **pre-configured template pack** on signup containing:

- Common line items with regional pricing
- Standard job types and durations
- Required permit checklists
- Safety protocol templates
- Material calculators specific to the trade
- Common photo documentation requirements
- Industry-specific compliance checklists
- Warranty terms templates
- Seasonal workflow adjustments

---

## 3. CORE PLATFORM ARCHITECTURE

### Platform Targets

- **Primary:** iOS (Swift/SwiftUI), Android (Kotlin/Compose)
- **Secondary:** Progressive Web App (tablet/desktop for office use)
- **Admin Portal:** Web dashboard for crew management and reporting
- **API:** Public REST + GraphQL API for integrations

### Design Principles

1. **One-thumb operation** — every critical action reachable with one thumb on a 6.1" screen
2. **Glove-friendly** — large touch targets (minimum 48x48dp), works with work gloves
3. **Sunlight readable** — high contrast mode, auto-brightness, dark/light adaptive
4. **3-tap rule** — any core action completed in 3 taps or fewer
5. **Voice-first alternative** — every action can be completed via voice command
6. **Offline by default** — every feature works without internet, syncs when connected
7. **Battery conscious** — GPS and sync optimized to not drain battery during 10-hour workdays
8. **Ruggedized UX** — designed for dirty hands, rain, dust, vibration

---

## 4. FEATURE SPECIFICATION — COMPLETE

### 4.1 ONBOARDING & IDENTITY

#### 4.1.1 30-Second Signup
- Phone number + SMS verification (primary — tradespeople don't use email)
- Email as optional secondary
- Google/Apple Sign-In as alternatives
- Trade selection from visual icon grid (not dropdown)
- Business name (optional, auto-generates from name + trade if skipped)
- Service area radius on map (drag to set)
- Import contacts from phone for instant customer list
- Skip everything option — functional app in 15 seconds

#### 4.1.2 Business Profile
- Business name, logo upload (or AI-generated from initials)
- License numbers (auto-validates against state databases where available)
- Insurance information with expiration tracking and renewal reminders
- Service area(s) — multiple zones with different rates supported
- Working hours with seasonal variations
- Holiday schedule
- Bio/about section for customer-facing profile
- Portfolio gallery (auto-populated from job photos over time)
- Certifications and specializations
- Years of experience
- Languages spoken
- BBB rating link
- Social media links
- Google Business Profile integration

#### 4.1.3 Team Setup (Crew Plans)
- Invite via SMS link (no app install required for basic use)
- Role assignment: Owner, Admin, Technician, Apprentice, Office Manager
- Skill/trade tags per team member
- Availability calendar per team member
- Hourly rate per team member (for job costing)
- GPS tracking consent and configuration
- Tool/vehicle assignment
- Certification tracking per team member
- Performance metrics visibility settings

#### 4.1.4 Guided Setup Wizard
- Trade-specific setup flow (plumber sees different questions than painter)
- Import existing customers from CSV, vCard, or phone contacts
- Connect payment processor (Stripe/Square) in 2 taps
- Set default hourly rate and markup percentages
- Upload logo or generate one
- Preview customer-facing estimate/invoice template
- Tutorial overlay on first use of each feature (dismissible, never returns)

---

### 4.2 JOB MANAGEMENT — THE CORE ENGINE

#### 4.2.1 Job Creation
- **Quick Job:** Customer name + address + description → done in 10 seconds
- **Detailed Job:** Full form with all fields below
- **Voice Job:** "Hey FieldForge, new job for John Smith at 123 Main Street, leaking kitchen faucet, tomorrow at 2pm" → job created with confirmation
- **Text-to-Job:** Forward a customer text/SMS → AI parses into structured job
- **Email-to-Job:** Forward customer email → AI parses into structured job
- **Repeat Job:** Clone from previous with date adjustment
- **Recurring Job:** Weekly, biweekly, monthly, quarterly, annual with auto-scheduling
- **Multi-day Job:** Project spanning multiple days with per-day scheduling
- **Callback Job:** Linked to original job for warranty/follow-up tracking

#### 4.2.2 Job Fields (All Optional Except Customer + Address)
- Customer (existing or create new inline)
- Property/Service address (auto-complete with Google Places)
- Property type: Residential, Commercial, Industrial, Government, HOA
- Property details: Square footage, year built, # stories, access notes
- Job type (from trade-specific presets or custom)
- Description (text + voice-to-text)
- Priority: Emergency, High, Normal, Low
- Estimated duration
- Scheduled date/time + arrival window
- Assigned technician(s)
- Required skills/certifications
- Required tools/equipment
- Required materials (from inventory or supplier catalog)
- Permit requirements
- Access instructions (gate code, lockbox, contact on arrival)
- Customer-provided photos
- Internal notes (not visible to customer)
- Tags/labels (custom, unlimited)
- Source/referral tracking
- Warranty status
- Related jobs (parent/child for multi-phase projects)
- Budget/not-to-exceed amount
- PO number
- Contract reference

#### 4.2.3 Job Statuses & Workflow
- **Lead** → **Estimated** → **Approved** → **Scheduled** → **En Route** → **In Progress** → **Paused** → **Completed** → **Invoiced** → **Paid** → **Closed**
- Custom status creation
- Status change triggers:
  - "En Route" → auto-sends customer ETA notification with live tracking link
  - "In Progress" → starts time tracking
  - "Completed" → prompts for completion photos, generates invoice
  - "Paid" → triggers review request after 24 hours
- Bulk status updates for multiple jobs
- Status change audit trail with timestamps and GPS

#### 4.2.4 Job Board / Calendar Views
- **Day View:** Timeline with jobs as blocks, drag to reschedule
- **Week View:** Grid layout, color-coded by trade/technician/status
- **Month View:** Overview with job counts per day
- **Map View:** All jobs on map with optimized route overlay
- **List View:** Sortable, filterable table (best for office/desktop)
- **Kanban View:** Drag cards between status columns
- **Agenda View:** Simple chronological list for today/tomorrow
- Unscheduled jobs queue (sidebar/drawer)
- Technician workload visualization (hours booked vs capacity)
- Weather overlay on calendar (affects outdoor trades significantly)
- Traffic-adjusted time blocks
- Drag-and-drop rescheduling with conflict detection
- Multi-technician job assignment with split views
- Color coding: by status, by trade, by technician, by priority, by customer type

#### 4.2.5 Job Execution (On-Site)
- **One-tap check-in** with GPS verification
- Automatic travel time logging (from previous job or home base)
- Time tracking with start/pause/resume/stop
- Material usage logging (scan barcode or select from inventory)
- Photo capture with automatic:
  - Timestamp watermark
  - GPS location watermark
  - Job number watermark
  - Before/during/after categorization
  - Damage documentation mode
  - Measurement overlay
- Video capture (up to 2 minutes per clip) for complex documentation
- Voice notes attached to job
- Checklist execution (trade-specific, custom, or regulatory)
- Safety checklist completion (required before work starts if configured)
- Digital signature capture (customer approval to proceed)
- Change order creation mid-job with customer approval flow
- Material request from supplier (in-app if integrated)
- Live chat with office/crew about the job
- Equipment readings/measurements logging
- Issue flagging (found additional problems)
- Parts ordering trigger
- Permit inspection scheduling
- Customer walkthrough documentation

#### 4.2.6 Job Completion
- Completion photos (required or optional per job type)
- Work summary auto-generated from time logs, materials, photos
- Customer sign-off (digital signature on device)
- Customer satisfaction quick-rating (1-5 stars, optional comment)
- Auto-generate invoice from job data
- Collect payment immediately or send invoice
- Schedule follow-up if needed
- Warranty terms application
- Request Google/Yelp review (one-tap, deep links to review pages)
- Auto-archive job with all documentation
- Post-job notes for future reference

---

### 4.3 CUSTOMER RELATIONSHIP MANAGEMENT (CRM)

#### 4.3.1 Customer Profiles
- Name, phone (primary + secondary), email
- Multiple properties/addresses per customer
- Property-specific notes and equipment logs
- Complete job history with photos, invoices, notes
- Equipment/appliance registry per property (model, serial, install date, warranty)
- Communication history (all texts, emails, calls logged)
- Payment history and outstanding balance
- Preferred communication method
- Preferred scheduling preferences
- Referral source tracking
- Customer lifetime value (auto-calculated)
- Tags/segments (VIP, commercial, property manager, etc.)
- Do-not-service flag with reason
- Credit terms (Net 15, Net 30, COD)
- Tax exemption status
- Emergency contact
- Gate codes, lockbox codes, alarm codes (encrypted storage)
- Pet information (important for in-home service)
- Preferred technician
- Anniversary/signup date for loyalty offers
- Notes pinned to top (critical info visible on every visit)

#### 4.3.2 Customer Communication
- **In-app messaging** (SMS-based, customer doesn't need the app)
- Automated appointment reminders (1 day + 1 hour before)
- "On my way" notification with live ETA and tracking map
- "Job complete" notification with summary and invoice
- Automated follow-up messages (3 days, 30 days, 90 days post-job)
- Review request automation (customizable timing and platform)
- Seasonal service reminders (e.g., "Time for your annual HVAC maintenance")
- Happy birthday/holiday messages (optional)
- Bulk messaging for promotions/announcements
- Two-way SMS conversations logged to customer record
- Email templates for estimates, invoices, follow-ups
- Call logging (tap to call, auto-log duration)
- Voicemail drop (pre-recorded message, one tap)
- Do Not Contact compliance (TCPA)
- Unsubscribe handling
- Message scheduling (send at optimal time)
- Template variables ({{customer_name}}, {{job_date}}, etc.)

#### 4.3.3 Customer Portal (Web)
- Unique link per customer (no login required, token-based access)
- View upcoming appointments
- View job history with photos
- View and pay invoices online
- Request new service
- Approve/decline estimates
- Sign documents digitally
- Download receipts/documentation for insurance
- Leave reviews
- Update contact information
- View technician profile and ETA
- Chat with assigned technician

---

### 4.4 ESTIMATING & PROPOSALS

#### 4.4.1 Estimate Builder
- **Quick Estimate:** Line items + quantities + rates → total in seconds
- **Detailed Proposal:** Multi-page document with:
  - Company branding (logo, colors, contact info)
  - Scope of work description
  - Line items grouped by phase/area/trade
  - Material specifications with links/images
  - Labor breakdown
  - Permit fees
  - Equipment rental
  - Markup/margin (hidden from customer)
  - Discount application (percentage or flat)
  - Tax calculation (auto by jurisdiction)
  - Payment terms
  - Timeline/milestones
  - Warranty terms
  - Terms and conditions
  - Digital signature acceptance
  - Expiration date
- **Good/Better/Best Options:** Present 3 tiers for upselling
- **Template Library:** Save and reuse estimates by job type
- Trade-specific line item catalogs with regional pricing
- Material cost integration with supplier APIs (where available)
- Auto-calculate labor from estimated hours × rate
- Historical pricing suggestions (based on similar past jobs)
- Competitor pricing context (optional market data)
- Financing options display (partner with GreenSky, Mosaic, etc.)
- Estimate versioning (v1, v2, v3 tracked)
- Internal cost vs customer price tracking (margin visibility)
- Multi-currency support (for border-area trades)

#### 4.4.2 Estimate Delivery & Approval
- Send via SMS link, email, or both
- Customer views on web (no app needed)
- Customer can approve, request changes, or decline
- E-signature capture on approval
- Deposit collection on approval (configurable percentage)
- Auto-convert approved estimate to scheduled job
- Estimate follow-up reminders (if no response in X days)
- Estimate win/loss analytics
- Reason for decline tracking

#### 4.4.3 AI Photo Estimation (Flagship Feature)
- Snap photo(s) of the job site/issue
- AI analyzes image and identifies:
  - Type of work needed
  - Scope/scale of the job
  - Materials likely required
  - Potential complications
- Generates suggested line items with pricing ranges
- User adjusts/confirms and sends to customer
- Continuously learns from user's pricing history
- Works with common trade scenarios:
  - Water damage → remediation line items
  - Cracked drywall → repair and paint line items
  - Old electrical panel → upgrade line items
  - Faded deck → staining/refinishing line items
  - Roof damage → repair/replacement line items
  - Bathroom demo → renovation line items
  - Overgrown yard → landscaping line items
  - Paint peeling → prep and paint line items

---

### 4.5 INVOICING & PAYMENTS

#### 4.5.1 Invoice Generation
- **Auto-generate from completed job** (time + materials + photos)
- **Manual creation** for non-job-related billing
- **Recurring invoices** for maintenance contracts
- Branded invoice template (logo, colors, business info)
- Line items from estimate or job materials log
- Time and materials billing (auto from time tracking)
- Flat rate billing
- Milestone billing for large projects
- Progress billing (percentage of total)
- Deposit invoicing
- Change order invoicing
- Tax calculation by jurisdiction (auto-lookup)
- Multi-tax support (state + county + city)
- Discount codes and promotional pricing
- Late fee configuration (auto-apply after grace period)
- Lien waiver generation (conditional and unconditional)
- PDF generation with professional formatting
- Batch invoicing (end of week/month)
- Invoice numbering (customizable format)
- PO number reference
- Notes/memo field
- Photo attachment (before/after)
- Terms and conditions footer
- Multiple payment method display (card, check, cash, financing)

#### 4.5.2 Payment Collection
- **Tap-to-Pay:** NFC payment via Stripe Terminal / Square Reader
- **Card on File:** Charge saved card with one tap
- **Online Payment:** Customer pays via invoice link (card, ACH, Apple Pay, Google Pay)
- **Cash/Check Logging:** Record offline payments with receipt generation
- **Partial Payments:** Accept deposits, progress payments, splits
- **Payment Plans:** Set up installment schedules
- **Financing:** Partner integration (GreenSky, Mosaic, Hearth) for large jobs
- **ACH/Bank Transfer:** Lower processing fees for larger amounts
- **Venmo/Zelle Integration:** Log payments from peer-to-peer platforms
- **Tips:** Optional tip prompt on digital payments
- Automatic receipt generation and delivery
- Payment reconciliation with bank feed
- Dispute/chargeback management
- Refund processing
- Multi-currency support
- Sales tax remittance reporting

#### 4.5.3 Accounts Receivable
- Aging report (current, 30, 60, 90, 120+ days)
- Automated payment reminders (configurable cadence)
- Escalation sequences (friendly → firm → final notice)
- Statement generation (monthly customer statements)
- Collections integration (third-party collections API)
- Lien filing assistance (state-specific workflows)
- Bad debt write-off tracking
- Interest/late fee calculations
- Customer credit limit management
- Deposit tracking and application
- Overpayment handling (credit on account)

---

### 4.6 SCHEDULING & DISPATCH

#### 4.6.1 Calendar System
- Interactive calendar with day/week/month/map views
- Drag-and-drop scheduling and rescheduling
- Appointment windows (e.g., 8am-12pm) vs exact times
- Buffer time between jobs (configurable, auto-calculated by distance)
- Overtime alerts and prevention
- Holiday/blackout date management
- Weather-aware scheduling (outdoor jobs flagged for bad weather)
- Customer availability preferences
- Recurring appointment scheduling
- Waitlist management for cancellations
- Emergency/priority job insertion with cascade rescheduling
- Tentative/confirmed appointment states
- Calendar sync (Google Calendar, Apple Calendar, Outlook)
- Public booking page (customer self-scheduling)

#### 4.6.2 Smart Dispatch
- **AI Route Optimization:**
  - Optimize daily route for minimum drive time
  - Consider job duration, priority, and customer windows
  - Real-time re-routing for traffic, cancellations, emergencies
  - Multi-stop optimization (TSP solver)
  - Vehicle capacity constraints (for trades carrying heavy equipment)
- **Technician Matching:**
  - Match jobs to technicians by: skill, certification, proximity, availability, customer preference
  - Load balancing across crew
  - Efficiency scoring per technician per job type
- **Dispatch Board:**
  - Real-time view of all technicians on map
  - Live GPS tracking (when in transit)
  - Status indicators (available, en route, on job, on break, off duty)
  - One-tap reassignment
  - Estimated completion times
  - Next job queuing

#### 4.6.3 GPS & Navigation
- Turn-by-turn navigation integration (Google Maps, Apple Maps, Waze)
- Multi-stop route with job sequence
- Traffic-aware ETAs updated in real-time
- Mileage tracking (auto-log for tax deduction)
- Service area boundary enforcement
- Drive time between jobs calculation
- Fuel cost estimation per route
- Vehicle tracking history (for fleet management)
- Geofencing for auto check-in/check-out
- Proximity alerts (near a customer who needs follow-up)

---

### 4.7 INVENTORY & MATERIALS MANAGEMENT

#### 4.7.1 Inventory Tracking
- Vehicle inventory per technician/truck
- Warehouse/shop inventory
- Real-time stock levels with low-stock alerts
- Barcode/QR code scanning for inventory management
- Material usage auto-deducted from jobs
- Re-order points and auto-purchase orders
- Multi-location inventory tracking
- Inventory valuation (FIFO, LIFO, average cost)
- Shrinkage/loss tracking
- Transfer between locations/vehicles
- Physical inventory count workflow
- Inventory photos for identification
- Custom units of measure
- Material categories and subcategories
- Preferred suppliers per material
- Price history tracking

#### 4.7.2 Supplier Integration
- Supplier directory with contact info
- Purchase order creation and tracking
- Supplier pricing catalogs
- Price comparison across suppliers
- Order history and spend analytics
- Delivery tracking
- Return/defect management
- Supplier payment tracking
- Integration with major distributors:
  - Ferguson (plumbing, HVAC)
  - Graybar (electrical)
  - HD Supply
  - Johnstone Supply (HVAC)
  - Sherwin-Williams (paint)
  - Benjamin Moore (paint)
  - Home Depot Pro
  - Lowe's Pro Supply
  - Grainger
  - Fastenal
  - ABC Supply (roofing)
  - SRS Distribution (roofing)
  - 84 Lumber (carpentry)
  - BlueLinx (lumber)

#### 4.7.3 Equipment & Tool Management
- Tool/equipment registry with serial numbers
- Maintenance schedules and logging
- Calibration tracking (for trades requiring it)
- Assignment tracking (who has what)
- Replacement value for insurance
- Check-out/check-in system
- Lost/stolen reporting
- Rental equipment tracking with return dates
- Tool warranty tracking
- Depreciation calculation

---

### 4.8 REPORTING & ANALYTICS

#### 4.8.1 Financial Reports
- Revenue by period (day, week, month, quarter, year)
- Revenue by trade/service type
- Revenue by technician
- Revenue by customer
- Revenue by property type (residential vs commercial)
- Profit and loss by job
- Profit and loss aggregate
- Average job value trending
- Materials cost vs markup analysis
- Labor cost analysis
- Overhead allocation
- Cash flow forecasting
- Outstanding receivables summary
- Collection rate metrics
- Discount impact analysis
- Refund/credit analysis
- Tax liability estimation
- Commission calculations
- Seasonal revenue patterns

#### 4.8.2 Operational Reports
- Jobs completed per period
- Average job duration by type
- First-time fix rate
- Callback/warranty rate
- Technician utilization (billable hours / available hours)
- Technician efficiency (estimated time vs actual time)
- Drive time vs on-job time ratio
- Cancellation/no-show rates
- Customer acquisition cost
- Customer retention rate
- Average response time (lead to estimate)
- Estimate-to-job conversion rate
- Average estimate value
- Inventory turnover
- Vehicle utilization
- On-time arrival percentage
- Schedule adherence
- Overtime tracking

#### 4.8.3 Customer Reports
- New customers per period
- Repeat customer rate
- Customer lifetime value
- Average revenue per customer
- Customer satisfaction scores
- Review metrics (average rating, count, trend)
- Referral tracking and attribution
- Churn rate
- Customer segment analysis
- Geographic heat map of customer density
- Top customers by revenue
- At-risk customers (declining activity)

#### 4.8.4 Dashboard
- Real-time KPI dashboard (customizable widgets)
- Goal tracking with visual progress
- Comparative period analysis (this month vs last month, this year vs last year)
- Benchmark against industry averages (anonymized aggregate data)
- Morning briefing: today's schedule, outstanding invoices, overdue items
- Weekly digest email with key metrics
- Anomaly detection alerts (unusual spend, sudden revenue drop, etc.)

---

### 4.9 DOCUMENT MANAGEMENT

#### 4.9.1 Photo Management
- Unlimited photo storage per job
- Automatic organization by job/customer/date
- Before/during/after categorization (manual or AI-suggested)
- Timestamp, GPS, and job number watermarking
- Photo annotation (draw arrows, circles, text on photos)
- Side-by-side before/after comparison view
- Photo albums for customer presentation
- Bulk photo operations (tag, move, delete)
- Photo quality settings (balance quality vs storage/bandwidth)
- HEIF/HEIC support with auto-conversion for sharing
- Photo compression for offline sync efficiency

#### 4.9.2 Document Storage
- Contracts and agreements
- Permits and inspection reports
- Insurance certificates
- License documents
- Lien waivers
- Safety data sheets (SDS/MSDS)
- Manufacturer warranties
- Customer-signed approvals
- Government/regulatory filings
- Vehicle registrations
- Equipment manuals
- Training certificates
- Union cards/documentation
- PDF generation from any document type
- Document expiration tracking with renewal alerts
- Document sharing via secure links
- Folder organization (by customer, by project, by type)

#### 4.9.3 Contract & Agreement Management
- Contract templates (service agreement, maintenance contract, subcontractor agreement)
- E-signature integration (DocuSign-level, built in)
- Contract versioning and amendment tracking
- Auto-renewal management
- Contract value and billing schedule tracking
- Expiration alerts
- Custom terms and conditions
- Multi-party signing workflows
- Contract templates by trade

---

### 4.10 MARKETING & GROWTH TOOLS

#### 4.10.1 Online Presence
- **FieldForge Public Profile Page** — SEO-optimized business page
  - Business info, services, service area
  - Photo gallery from completed jobs
  - Customer reviews aggregated
  - Online booking widget
  - "Request Estimate" form
  - Google Business Profile sync
- **Website Widget** — embeddable booking/estimate request for existing websites
- **QR Code Generator** — for truck wraps, business cards, yard signs
  - Links to booking page, review page, or estimate request
- **Social Media Content Generator**
  - Auto-generate before/after posts from job photos
  - Review highlight graphics
  - Seasonal promotion templates
  - Branded templates with logo and colors

#### 4.10.2 Review Management
- Automated review requests post-job
- Multi-platform support (Google, Yelp, Facebook, Angi, BBB, Nextdoor)
- Review monitoring and response from one inbox
- Negative review interception (private feedback first, redirect satisfied customers to public)
- Review response templates (AI-assisted)
- Review analytics (rating trend, response rate, keyword analysis)
- Review showcase on profile page and proposals
- QR code for in-person review requests

#### 4.10.3 Referral System
- Customer referral program (give $X, get $X)
- Technician referral tracking
- Realtor/property manager partner program
- Builder/GC partner program
- Referral link generation and tracking
- Automated referral credit application
- Referral leaderboard

#### 4.10.4 Email & SMS Marketing
- Customer segmentation for targeted campaigns
- Seasonal service reminders
- Holiday promotions
- Maintenance plan upsells
- Win-back campaigns for inactive customers
- New service announcements
- Template library with trade-specific content
- Campaign analytics (open rate, click rate, conversion)
- A/B testing for messages
- TCPA/CAN-SPAM compliance built-in
- Unsubscribe management

---

### 4.11 MAINTENANCE AGREEMENTS & RECURRING SERVICES

#### 4.11.1 Service Plans
- Create maintenance plan templates (e.g., "Annual HVAC Tune-Up Plan")
- Pricing: monthly, quarterly, annual billing
- Included services and visit frequency
- Parts/materials coverage
- Priority scheduling for plan members
- Discount on additional work
- Plan enrollment via customer portal or in-person
- Auto-renewal with notification
- Plan cancellation and proration
- Family/multi-property plans

#### 4.11.2 Recurring Job Management
- Auto-scheduling of recurring visits
- Seasonal adjustment (e.g., HVAC: spring tune-up + fall tune-up)
- Technician assignment consistency (same tech each visit)
- Automated customer reminders
- Visit tracking against plan terms
- Plan profitability analysis
- Churn prediction for at-risk plan customers

---

### 4.12 SUBCONTRACTOR MANAGEMENT

#### 4.12.1 Subcontractor Directory
- Sub profiles with trades, rates, availability
- Insurance and license verification
- W-9 collection and storage
- Performance ratings from past work
- Preferred subs per job type/area
- Sub availability calendar

#### 4.12.2 Sub Assignment & Payment
- Assign subs to jobs or portions of jobs
- Sub receives job details via SMS/app
- Time and material tracking for subs
- Sub invoicing and payment tracking
- 1099 reporting (year-end generation)
- Margin tracking on sub work
- Sub agreement/contract management
- Lien waiver collection from subs

---

### 4.13 FLEET & VEHICLE MANAGEMENT

#### 4.13.1 Vehicle Tracking
- Vehicle registry (make, model, year, VIN, license plate)
- GPS tracking (opt-in, configurable hours)
- Mileage tracking (auto from GPS)
- Fuel expense logging
- Maintenance schedule (oil change, tires, inspection)
- Maintenance history and cost tracking
- Vehicle assignment to technician
- Vehicle inspection checklists (pre-trip, FMCSA if applicable)
- Insurance and registration tracking with expiration alerts
- Accident/incident reporting

#### 4.13.2 Fleet Optimization
- Fuel cost analysis per vehicle and per mile
- Vehicle total cost of ownership
- Maintenance cost trending
- Vehicle utilization rates
- Replacement planning (age/mileage thresholds)
- Route efficiency by vehicle
- Carbon footprint tracking (for companies pursuing green certifications)

---

### 4.14 COMPLIANCE & SAFETY

#### 4.14.1 Permit Management
- Permit requirement detection by job type and jurisdiction
- Permit application tracking (applied, issued, inspecting, passed, failed)
- Permit cost tracking and billing to customer
- Inspection scheduling
- Inspector contact information
- Permit document storage
- Permit renewal tracking
- Automatic permit cost inclusion in estimates

#### 4.14.2 Safety Management
- Job safety checklists (OSHA-aligned)
- Toolbox talk templates and logging
- Incident reporting workflow
- Near-miss reporting
- Safety training tracking per employee
- PPE assignment and tracking
- SDS/MSDS library
- Emergency procedure documentation
- OSHA 300 log maintenance
- Drug test compliance tracking
- Workers' comp claim management

#### 4.14.3 Licensing & Insurance
- License tracking with expiration alerts (90, 60, 30 day warnings)
- Multi-state license management
- Continuing education (CE) credit tracking
- Insurance certificate management
- Auto-generation of insurance certificates for customers/GCs
- Bond tracking
- Workers' comp policy management
- General liability tracking
- Vehicle insurance tracking
- Professional liability/E&O tracking
- Umbrella policy tracking

---

### 4.15 INTEGRATIONS

#### 4.15.1 Accounting
- QuickBooks Online (two-way sync: invoices, payments, expenses, customers)
- QuickBooks Desktop (export/import)
- Xero (two-way sync)
- FreshBooks
- Wave Accounting
- Sage
- Chart of accounts mapping
- Transaction categorization
- Bank reconciliation support
- Multi-entity support

#### 4.15.2 Payment Processing
- Stripe (primary: cards, ACH, Apple Pay, Google Pay, tap-to-pay)
- Square (alternative: card reader, POS, invoicing)
- PayPal / Venmo (logging)
- Zelle (logging)
- GreenSky (customer financing)
- Mosaic (solar financing)
- Hearth (home improvement financing)
- Wisetack (consumer financing)

#### 4.15.3 Communication
- Twilio (SMS backbone)
- SendGrid (email backbone)
- Mailchimp (marketing automation)
- Google Workspace
- Microsoft 365
- Slack (team communication)
- VoIP integration (RingCentral, Grasshopper)

#### 4.15.4 Mapping & Location
- Google Maps Platform (routing, places, geocoding)
- Apple MapKit (iOS native maps)
- Mapbox (custom map experiences)
- HERE Technologies (routing optimization)
- Waze (navigation deep link)

#### 4.15.5 Industry-Specific
- Angi Leads (lead import)
- Thumbtack (lead import)
- HomeAdvisor (lead import)
- Yelp (reviews, leads)
- Google Local Services Ads (lead import)
- Nextdoor (business listing)
- BuilderTrend (for GC projects)
- PlanGrid/Autodesk (blueprint access)
- CompanyCam (photo documentation)
- ScanLife/barcode APIs (inventory)
- ENERGY STAR (HVAC certification data)
- Manufacturer warranty systems

#### 4.15.6 Zapier / Make / API
- Full REST API for custom integrations
- GraphQL API for flexible queries
- Webhook support for real-time events
- Zapier app (triggers: job created, job completed, payment received, estimate approved; actions: create job, create customer, send message)
- Make (Integromat) module
- API rate limiting and authentication (OAuth 2.0 + API keys)
- SDKs: JavaScript, Python, Swift, Kotlin
- API documentation with interactive playground

---

### 4.16 NOTIFICATIONS & ALERTS

#### 4.16.1 Push Notifications (Tradesperson)
- New job assigned
- Schedule change
- Customer approved estimate
- Payment received
- Customer message received
- Job reminder (morning briefing)
- Low inventory alert
- License/insurance expiring
- Negative review posted
- Overdue invoice
- Upcoming recurring job
- Weather alert affecting scheduled outdoor job
- Emergency dispatch

#### 4.16.2 Customer Notifications (SMS/Email)
- Appointment confirmation
- Appointment reminder (24hr + 1hr)
- Technician on the way (with ETA and tracking link)
- Job complete summary
- Invoice / payment request
- Payment confirmation / receipt
- Review request
- Estimate ready for review
- Service plan renewal
- Seasonal maintenance reminder
- Promotional offers (opted-in only)

#### 4.16.3 Office/Admin Notifications
- New lead received
- Estimate pending review
- Job flagged by technician
- Schedule conflict detected
- Overtime approaching
- Cash flow alert
- Large payment received
- Customer complaint
- Compliance item expiring
- Weekly/monthly report ready

---

### 4.17 SETTINGS & CUSTOMIZATION

#### 4.17.1 Business Settings
- Company information
- Branding (logo, colors, fonts)
- Default rates (hourly, markup percentages)
- Tax configuration (rates, exemptions)
- Invoice numbering format
- Estimate numbering format
- Payment terms defaults
- Working hours and availability
- Service area configuration
- Holiday calendar
- Currency and locale
- Time zone
- Language (English, Spanish, Portuguese, French — critical for trades workforce)

#### 4.17.2 Template Customization
- Estimate template (layout, fields, branding)
- Invoice template
- Receipt template
- Customer communication templates (10+ per category)
- Contract templates
- Checklist templates
- Report templates

#### 4.17.3 Automation Rules
- If [trigger] then [action] builder:
  - Job completed → Generate invoice → Send to customer
  - Payment received → Send receipt → Request review after 24hr
  - Estimate approved → Schedule job → Send confirmation
  - Lead received → Send acknowledgment → Assign to available tech
  - Invoice overdue 7 days → Send reminder
  - Invoice overdue 30 days → Send firm notice
  - Customer inactive 90 days → Send win-back offer
  - Custom rules with AND/OR logic

---

### 4.18 ACCESSIBILITY & LOCALIZATION

#### 4.18.1 Accessibility
- VoiceOver (iOS) and TalkBack (Android) full support
- Voice command for all critical actions
- Large text mode
- High contrast mode
- Color blind safe color schemes
- One-handed operation mode
- Haptic feedback for confirmations
- Screen reader optimized navigation

#### 4.18.2 Localization
- English (US) — primary
- Spanish (US) — critical (largest secondary language in US trades)
- Portuguese (BR) — significant trades population
- French (CA) — Canadian market
- Chinese (Simplified) — growing trades workforce
- Korean — significant trades population in certain metro areas
- Vietnamese — significant trades population in certain metro areas
- Polish — significant trades population in certain metro areas
- Right-to-left support (Arabic, Hebrew) — future
- Currency localization
- Date/time format localization
- Phone number format localization
- Address format localization

---

### 4.19 SECURITY & PRIVACY

#### 4.19.1 Data Security
- AES-256 encryption at rest
- TLS 1.3 encryption in transit
- SOC 2 Type II compliance
- PCI DSS compliance (payment data)
- Zero-knowledge encryption for sensitive fields (gate codes, alarm codes)
- Biometric authentication (Face ID, Touch ID, fingerprint)
- PIN/passcode app lock
- Session management and auto-timeout
- IP allowlisting for admin portal
- Data residency options (US, EU, CA)
- Regular penetration testing
- Bug bounty program
- Incident response plan

#### 4.19.2 Privacy
- GDPR compliance
- CCPA compliance
- Customer data export (right to portability)
- Customer data deletion (right to be forgotten)
- Consent management
- Cookie/tracking transparency
- Privacy policy generator for the tradesperson's business
- Data processing agreements (DPA) for enterprise
- Minimal data collection principle
- Anonymous analytics option

#### 4.19.3 Backup & Recovery
- Automated daily backups
- Point-in-time recovery
- Cross-region replication
- 99.99% uptime SLA
- Disaster recovery plan with 1-hour RPO, 4-hour RTO
- Data export in standard formats (CSV, JSON, PDF)
- Account deactivation with data retention period
- Full account data download

---

## 5. AI/ML FEATURE SUITE

### 5.1 AI Photo Estimation (Core Differentiator)
- Multi-photo analysis for comprehensive job assessment
- Damage severity classification
- Material identification from photos
- Measurement estimation from photos (using reference objects)
- Historical price matching from similar jobs
- Regional pricing adjustments
- Material quality/grade recommendations
- Complication detection (asbestos indicators, structural concerns, code violations)
- Confidence scoring on estimates
- Continuous learning from user corrections
- Trade-specific models (plumbing damage vs electrical hazard vs roof damage)

### 5.2 Smart Scheduling AI
- Predict job duration based on type, complexity, technician
- Optimize daily routes for minimum travel time
- Balance workload across crew
- Suggest best time slots for customer satisfaction
- Predict cancellation/no-show risk
- Weather impact prediction for outdoor jobs
- Traffic pattern learning for accurate ETAs
- Emergency job insertion with minimal disruption

### 5.3 Predictive Business Intelligence
- Revenue forecasting (weekly, monthly, quarterly)
- Cash flow prediction
- Seasonal demand forecasting
- Customer churn prediction
- Equipment failure prediction (based on age, usage, service history)
- Parts/material demand forecasting for inventory optimization
- Optimal pricing recommendations
- Customer lifetime value prediction
- Job profitability prediction before accepting
- Lead quality scoring

### 5.4 Natural Language Processing
- Voice-to-job creation (full NLP parsing)
- SMS/email-to-job parsing
- Smart search across all data (natural language queries)
- Auto-categorization of expenses and transactions
- Customer sentiment analysis from communications
- Automated review response drafting
- Meeting/call notes transcription and summarization
- Work order generation from customer descriptions

### 5.5 Computer Vision
- Photo-based measurement (room dimensions, pipe sizes, wire gauges)
- Equipment model/serial number OCR from photos
- License plate recognition for fleet
- Barcode/QR scanning for inventory
- Document scanning and OCR (receipts, permits, business cards)
- Photo quality assessment (blur detection, lighting check)
- Safety hazard detection in job site photos
- Before/after auto-matching

### 5.6 AI Business Assistant
- "Ask FieldForge" — natural language business queries
  - "What was my revenue last quarter?"
  - "Which customers haven't been serviced in 6 months?"
  - "What's my most profitable job type?"
  - "Schedule Mrs. Johnson's annual maintenance"
- Daily briefing generation
- Anomaly detection and alerting
- Competitive pricing analysis
- Upsell/cross-sell recommendations per customer
- Automated report generation
- Chatbot for customer portal (FAQ + booking + status)

---

## 6. OFFLINE-FIRST ARCHITECTURE

### 6.1 Offline Capabilities (Everything Critical Works Without Internet)
- View full schedule and job details
- Navigate to jobs (cached maps + GPS)
- Start/pause/stop time tracking
- Log materials used
- Capture photos and videos
- Write notes (text and voice)
- Execute checklists
- Capture customer signatures
- Create new jobs
- Create quick estimates
- View customer information
- View inventory
- Record cash/check payments
- Complete all job execution workflows

### 6.2 Sync Engine
- Automatic sync when connectivity restored
- Conflict resolution with last-write-wins + manual merge for conflicts
- Incremental sync (only changed data)
- Background sync (doesn't interrupt user)
- Sync progress indicator
- Selective sync (prioritize today's jobs)
- Bandwidth-efficient photo sync (compress, queue, upload when on WiFi optional)
- Sync status per record
- Manual force-sync option
- Sync log for debugging
- Multi-device sync (phone + tablet)

### 6.3 Local Storage
- SQLite local database with encryption
- Photo storage with intelligent caching (recent + today's jobs)
- Map tile caching for service area
- Customer data caching (all active customers)
- Configurable storage limits
- Auto-cleanup of old cached data
- Storage usage monitoring

---

## 7. PAYMENT & FINANCIAL ENGINE

### 7.1 Payment Processing Features
- **Instant Payout:** Same-day or next-day access to funds (Stripe Instant Payouts)
- **Split Payments:** Customer pays with multiple methods
- **Recurring Billing:** Maintenance plans auto-charged monthly
- **Deposit Automation:** Auto-charge deposit when estimate approved
- **Payment Links:** Send standalone payment link via SMS
- **Terminal/Reader Support:** Stripe Reader M2, Square Reader, tap-to-pay on iPhone/Android
- **Multi-Currency:** USD, CAD, GBP, EUR (for cross-border trades)
- **Batch Payout:** Weekly payout schedule option for lower fees
- **Payout Reporting:** Reconcile payouts with invoices
- **Fee Transparency:** Show processing fees per transaction
- **Surcharge Option:** Pass processing fee to customer (where legally permitted)

### 7.2 Financial Management
- **Expense Tracking:**
  - Receipt scanning (OCR → auto-categorize)
  - Mileage deduction tracking
  - Tool and equipment expense logging
  - Material purchase tracking
  - Subcontractor payment tracking
  - Vehicle expense tracking
  - Insurance payment tracking
  - Software/subscription tracking
  - Home office deduction support
  - Categorization aligned with Schedule C
- **Tax Preparation:**
  - Quarterly estimated tax calculation
  - Year-end tax summary
  - 1099 generation for subcontractors
  - Sales tax collection and reporting
  - Mileage deduction summary
  - Equipment depreciation tracking
  - Export for CPA (QuickBooks, CSV)
- **Budgeting:**
  - Monthly budget setting and tracking
  - Category-level budgets
  - Budget vs actual variance reporting
  - Alerts when approaching budget limits

---

## 8. COMPLIANCE, LICENSING & INSURANCE

### 8.1 Jurisdiction-Aware Compliance
- License requirements by state and trade
- Continuing education requirements tracking
- Permit requirements by municipality and job type
- Code compliance checklists (NEC, IRC, IPC, IMC, IECC)
- EPA certifications (608, 609 for HVAC)
- Lead-RRP certification tracking
- Asbestos awareness certification
- Confined space entry requirements
- OSHA requirements tracking
- Workers' compensation requirements by state
- Business license renewal tracking

### 8.2 Insurance Management
- Certificate of Insurance (COI) generation
- Additional insured endorsement tracking
- Policy expiration monitoring
- Coverage amount validation against contract requirements
- Auto-send COI to GCs/property managers
- Insurance claim documentation support

---

## 9. ANALYTICS & BUSINESS INTELLIGENCE

### 9.1 Industry Benchmarking (Anonymized)
- Compare your metrics to similar businesses (same trade, same region, same size)
- Revenue per employee benchmark
- Customer acquisition cost benchmark
- Average job value benchmark
- Collection period benchmark
- Utilization rate benchmark
- Customer satisfaction benchmark
- Growth rate benchmark

### 9.2 Custom Report Builder
- Drag-and-drop report designer
- Custom date ranges
- Custom filters and grouping
- Calculated fields
- Chart types: bar, line, pie, funnel, scatter, table
- Scheduled report delivery (email)
- Export to PDF, CSV, Excel
- Share reports with team members (role-based)
- Saved report templates

---

## 10. GROWTH & VIRAL MECHANICS

### 10.1 Viral Loops
- **Customer Experience:** Every customer interaction is branded (estimate, invoice, receipt, tracking page) with "Powered by FieldForge" and signup CTA
- **Referral Program:** $50 credit for each referred tradesperson who subscribes
- **Subcontractor Network:** When subs receive job assignments, they see FieldForge value proposition
- **Property Manager Hub:** Property managers manage multiple trades through one portal → each trade they invite gets FieldForge
- **Supplier Partnerships:** Supplier order integration drives adoption
- **Review Widget:** Publicly visible reviews on Google with FieldForge branding
- **Job Complete Page:** Customer-facing completion page with FieldForge branding

### 10.2 Network Effects
- **Trade Network:** Plumber refers electrician refers HVAC tech → neighborhood trade network
- **Customer Network:** Homeowner uses one FieldForge tradesperson → discovers others
- **Data Network:** More jobs = better AI estimates = more value for all users
- **Marketplace Potential (Future):** Customers find tradespeople → FieldForge becomes the platform

---

## 11. MONETIZATION & PRICING TIERS

### 11.1 Free Trial
- 14-day full-feature trial
- No credit card required
- All features unlocked
- Import data stays if converted

### 11.2 Pricing Tiers

| Feature | Solo ($19/mo) | Crew ($49/mo) | Business ($99/mo) | Enterprise (Custom) |
|---------|:---:|:---:|:---:|:---:|
| Users | 1 | Up to 5 | Up to 15 | Unlimited |
| Jobs/month | Unlimited | Unlimited | Unlimited | Unlimited |
| Customers | Unlimited | Unlimited | Unlimited | Unlimited |
| Estimates & Invoices | Unlimited | Unlimited | Unlimited | Unlimited |
| Photo Storage | 10GB | 50GB | 200GB | Unlimited |
| AI Photo Estimates | 10/mo | 50/mo | Unlimited | Unlimited |
| Offline Mode | ✓ | ✓ | ✓ | ✓ |
| GPS Routing | ✓ | ✓ | ✓ | ✓ |
| Payment Collection | ✓ | ✓ | ✓ | ✓ |
| Scheduling | ✓ | ✓ | ✓ | ✓ |
| Customer Portal | ✓ | ✓ | ✓ | ✓ |
| Recurring Jobs | ✓ | ✓ | ✓ | ✓ |
| Dispatch Board | — | ✓ | ✓ | ✓ |
| Team GPS Tracking | — | ✓ | ✓ | ✓ |
| Inventory Management | — | Basic | Advanced | Advanced |
| Subcontractor Mgmt | — | — | ✓ | ✓ |
| Fleet Management | — | — | ✓ | ✓ |
| Custom Reports | — | — | ✓ | ✓ |
| API Access | — | — | ✓ | ✓ |
| QuickBooks Sync | ✓ | ✓ | ✓ | ✓ |
| Multi-Location | — | — | ✓ | ✓ |
| White Label | — | — | — | ✓ |
| Dedicated Support | — | — | Phone | Dedicated CSM |
| SLA | 99.9% | 99.9% | 99.95% | 99.99% |
| Onboarding | Self-serve | Video call | Dedicated | Dedicated team |

### 11.3 Revenue Add-Ons
- Payment processing margin: 2.0% on Stripe/Square transactions
- Additional AI photo estimates: $0.50 per estimate beyond plan limit
- Premium integrations: $10/mo per premium integration
- Additional storage: $5/mo per 50GB
- Branded customer portal: $15/mo
- Priority support: $25/mo (Solo/Crew plans)
- Data export/migration: $99 one-time
- Custom development: hourly rate

### 11.4 Annual Discount
- 20% off on annual billing (Solo: $182/yr, Crew: $470/yr, Business: $950/yr)

---

## 12. TECHNICAL INFRASTRUCTURE

### 12.1 Technology Stack Considerations
- **Mobile:** Native iOS (Swift/SwiftUI) + Native Android (Kotlin/Compose)
- **Web Dashboard/Portal:** SvelteKit 5 + TypeScript strict
- **API:** Rust/Axum (high performance, safety) OR Node.js/Fastify
- **Database:** PostgreSQL (primary) + SQLite (local/offline)
- **Real-time:** WebSockets via Rust/Tokio or Socket.io
- **Search:** Meilisearch or Typesense (instant, typo-tolerant)
- **Queue:** Redis + BullMQ for background jobs
- **Storage:** S3-compatible (photos, documents)
- **CDN:** CloudFront or Cloudflare R2
- **AI/ML:** Python microservices (FastAPI) for model inference
- **Maps:** Google Maps Platform + Mapbox
- **Payments:** Stripe (primary) + Square
- **SMS:** Twilio
- **Email:** SendGrid + AWS SES
- **Push:** Firebase Cloud Messaging + APNs
- **Auth:** Supabase Auth or Auth0
- **Infrastructure:** AWS (ECS/EKS) or Railway + Supabase
- **CI/CD:** GitHub Actions
- **Monitoring:** Datadog or Grafana + Sentry
- **Analytics:** PostHog (product) + Metabase (business)

### 12.2 Performance Targets
- App launch to usable: < 2 seconds
- Job creation: < 5 seconds
- Photo capture to saved: < 1 second
- Estimate generation: < 3 seconds (online), < 1 second (offline)
- Invoice generation: < 2 seconds
- Sync after reconnection: < 10 seconds for typical day's data
- AI photo analysis: < 8 seconds
- Search results: < 200ms
- API response time: < 100ms (p95)
- Push notification delivery: < 3 seconds
- GPS tracking update: every 30 seconds (en route), every 5 minutes (on job)
- Battery impact: < 10% per 8-hour workday with active GPS

### 12.3 Scale Targets
- 100K concurrent users
- 1M jobs/day processing
- 10M photos/day upload
- 500K payment transactions/day
- 10M push notifications/day
- 1PB total photo storage
- 99.99% uptime

---

## 13. NUCLEAR AI IMPLEMENTATION PROMPTS

Below are production-grade, Apple Principal Engineer ICT Level 7+ prompts for AI-assisted implementation of every major FieldForge module. Each prompt is designed to produce complete, zero-compromise, enterprise-grade code.

---

### PROMPT 1: OFFLINE-FIRST DATABASE & SYNC ENGINE

```
You are a Principal Engineer at Apple, ICT Level 7+. You are building the offline-first data layer for FieldForge, a mobile field service management app for tradespeople. This is the most critical infrastructure component — everything depends on it.

CONTEXT:
- FieldForge serves plumbers, electricians, HVAC techs, painters, carpenters, roofers, landscapers, and 15+ other trades
- Users work in basements, attics, crawl spaces, and rural areas with ZERO internet connectivity
- Every critical feature must work offline: job viewing, creation, time tracking, photo capture, checklist execution, customer signatures, estimates, notes
- When connectivity returns, data must sync automatically with conflict resolution
- Multi-device sync (phone + tablet) must be supported

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. LOCAL DATABASE SCHEMA (SQLite with encryption):
   - Design the complete normalized schema for ALL entities: jobs, customers, properties, estimates, invoices, line_items, payments, photos, documents, time_entries, materials_used, checklists, checklist_items, notes, signatures, team_members, vehicles, inventory, equipment, contracts, recurring_jobs, tags, custom_fields
   - Every table must have: id (UUID v7), created_at, updated_at, synced_at, sync_status (pending|syncing|synced|conflict), version (integer), deleted_at (soft delete)
   - Foreign key relationships with cascade rules
   - Indexes optimized for mobile query patterns (job lookup by date range, customer search, status filtering)
   - Full-text search indexes for customer name, job description, address
   - Encrypted columns for sensitive data (gate codes, alarm codes, financial data)

2. SYNC ENGINE:
   - Implement a complete CRDT-inspired sync protocol
   - Delta sync: only send changed records since last sync timestamp
   - Conflict resolution strategy: last-write-wins for most fields, manual merge for critical conflicts (e.g., two people editing same estimate)
   - Sync priority queue: payments > job status changes > photos > everything else
   - Bandwidth-efficient: binary protocol with compression
   - Resumable uploads for large payloads (photos)
   - Sync state machine: idle → checking → downloading → uploading → resolving_conflicts → complete
   - Background sync on iOS (BGTaskScheduler) and Android (WorkManager)
   - Sync progress observable for UI
   - Retry with exponential backoff
   - Network quality detection (WiFi vs cellular vs metered)
   - Photo sync strategy: thumbnail immediately, full resolution on WiFi
   - Complete audit trail of all sync operations

3. DATA ACCESS LAYER:
   - Repository pattern with protocol/interface for each entity
   - Reactive queries (Combine on iOS, Flow on Kotlin) — UI auto-updates when data changes
   - Query builder with type-safe predicates
   - Pagination support (cursor-based)
   - Batch operations with transaction support
   - Migration system for schema evolution
   - Data export in JSON and CSV formats

4. OFFLINE QUEUE:
   - Queue all mutations when offline
   - Persist queue to disk (survives app kill/crash)
   - Order-preserving execution on sync
   - Dependency resolution (e.g., create customer before creating job for that customer)
   - Failed operation handling with user notification

DELIVERABLES:
- Complete SQLite schema DDL
- Complete sync engine implementation (Swift for iOS OR Kotlin for Android — specify which and I will request the other)
- Complete data access layer with all repositories
- Unit tests for sync conflict resolution
- Integration tests for offline → online transition scenarios
- Documentation for the sync protocol

CODE STANDARDS:
- Zero warnings, zero force unwraps (Swift), zero unchecked casts (Kotlin)
- Complete error handling with typed errors
- Logging at every critical decision point
- Thread safety with actors (Swift) or coroutines (Kotlin)
- Memory efficient — this runs on phones with 3-4GB RAM alongside other apps
- Battery efficient — minimize disk I/O and network calls
```

---

### PROMPT 2: JOB MANAGEMENT & SCHEDULING ENGINE

```
You are a Principal Engineer at Apple, ICT Level 7+. You are building the complete job management and scheduling engine for FieldForge.

CONTEXT:
- This is the core of the app — tradespeople live in the job management and calendar screens
- Must support drag-and-drop scheduling, multi-view calendar, real-time dispatch, and AI-optimized routing
- Jobs go through a complete lifecycle: Lead → Estimated → Approved → Scheduled → En Route → In Progress → Paused → Completed → Invoiced → Paid → Closed
- Every state transition triggers side effects (notifications, time tracking, GPS updates, etc.)

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. JOB STATE MACHINE:
   - Implement a formal finite state machine for job lifecycle
   - Define all valid state transitions with guard conditions
   - Side effect handlers for each transition:
     - Lead → Estimated: generate estimate draft
     - Approved → Scheduled: add to calendar, send confirmation
     - Scheduled → En Route: start GPS tracking, send customer ETA
     - En Route → In Progress: stop navigation, start time tracking, geofence check-in
     - In Progress → Paused: pause time tracking, log pause reason
     - In Progress → Completed: stop time tracking, prompt completion photos, generate invoice draft
     - Completed → Invoiced: create and send invoice
     - Invoiced → Paid: record payment, send receipt, trigger review request (24hr delay)
   - State change audit trail with timestamp, user, GPS location
   - Undo last state change (within 5 minutes)
   - Bulk state changes (mark multiple jobs complete)

2. CALENDAR / SCHEDULING UI:
   - Day view: vertical timeline with job blocks showing customer, address, time, status
   - Week view: grid with columns per day, rows per technician
   - Month view: day cells with job count badges, color-coded
   - Map view: all jobs as pins with route overlay and sequence numbers
   - Kanban view: columns per status, drag cards between columns
   - Agenda view: simple chronological list for today/tomorrow
   - Drag-and-drop rescheduling with haptic feedback
   - Pinch to zoom on timeline
   - Conflict detection (overlapping jobs, exceeding working hours)
   - Buffer time visualization between jobs
   - Weather overlay (outdoor jobs flagged for rain/extreme temps)
   - Unscheduled jobs tray (swipe from edge to reveal)
   - Technician capacity heatmap
   - Pull-to-refresh
   - All views must render in < 16ms per frame (60fps)
   - Support for 500+ jobs on screen without jank

3. SMART SCHEDULING ALGORITHM:
   - Input: unscheduled jobs, technician availability, existing schedule, traffic patterns
   - Constraints: working hours, skill requirements, customer time windows, travel time, job duration
   - Optimization objective: minimize total travel time while respecting all constraints
   - Algorithm: implement a constraint-satisfaction solver with local search optimization
   - Support for manual overrides (pin jobs, exclude technicians)
   - Re-optimization on schedule changes
   - What-if analysis: "what happens if I add this emergency job?"
   - Emergency insertion: find best slot for high-priority job with minimal disruption

4. DISPATCH BOARD (Crew/Business plans):
   - Real-time technician map with live GPS
   - Status indicators per technician (available, en route, on job, break, off)
   - Job queue per technician
   - One-tap reassignment with notification to both technicians
   - ETA calculations using real-time traffic
   - Drag technician to job for assignment
   - Unassigned jobs panel
   - Capacity utilization bars per technician

5. RECURRING JOB ENGINE:
   - RRULE-compatible recurrence: daily, weekly, biweekly, monthly, quarterly, annually
   - Custom recurrence patterns (e.g., every 3rd Wednesday)
   - Seasonal recurrence (spring and fall only)
   - Auto-generate upcoming instances
   - Individual instance modification without affecting series
   - Series modification with options: this instance, this and future, all instances
   - Skip/cancel individual instances
   - End date or count-based termination
   - Auto-assign to preferred technician

DELIVERABLES:
- Complete job state machine implementation with all transitions and side effects
- Calendar UI components for all 6 views (SwiftUI or Jetpack Compose)
- Scheduling optimization algorithm
- Dispatch board implementation
- Recurring job engine
- Complete unit and integration tests
- Performance benchmarks showing 60fps calendar rendering with 500+ jobs

CODE STANDARDS:
- Unidirectional data flow architecture
- All state changes through the state machine — no direct property mutation
- Accessibility: full VoiceOver/TalkBack support on all calendar views
- Localization-ready: all strings externalized, RTL support
- Gesture handling: drag, pinch, long-press, swipe — all with proper hit testing and conflict resolution
```

---

### PROMPT 3: AI PHOTO ESTIMATION ENGINE

```
You are a Principal Engineer at Apple, ICT Level 7+, with deep expertise in computer vision and ML. You are building the AI photo estimation engine for FieldForge — the flagship differentiating feature.

CONTEXT:
- Tradespeople take photos of job sites (water damage, broken fixtures, old panels, damaged roofs, peeling paint, rotted wood, etc.)
- The AI analyzes photos and generates suggested line items with estimated pricing
- This must work across ALL trades: plumbing, electrical, HVAC, painting, carpentry, roofing, landscaping, flooring, concrete, etc.
- Accuracy improves over time as users accept, modify, or reject suggestions
- Regional pricing variations must be accounted for (NYC plumber rates ≠ rural Oklahoma rates)

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. COMPUTER VISION PIPELINE:
   - Multi-image input (1-10 photos per estimation)
   - Image preprocessing: auto-rotate, brightness/contrast normalization, quality check
   - Scene classification: identify trade domain (plumbing, electrical, roofing, etc.)
   - Object detection: identify specific items, fixtures, materials, damage types
   - Damage/condition assessment: severity classification (minor, moderate, major, critical)
   - Measurement estimation: use reference objects for scale (doors, outlets, standard fixtures)
   - Safety hazard detection: mold, asbestos indicators, structural damage, electrical hazards
   - Material identification: pipe type, wire gauge, wood species, flooring material
   - Model architecture: fine-tuned vision transformer (ViT) or EfficientNet backbone with custom classification heads
   - On-device inference option for basic classification (CoreML/TensorFlow Lite)
   - Server-side inference for detailed analysis

2. ESTIMATION GENERATION:
   - Map CV outputs to trade-specific line item catalog
   - Line item structure: description, quantity, unit, unit_price, total, category
   - Categories: labor, materials, equipment, permits, disposal, overhead
   - Pricing engine:
     - Base pricing from national trade databases
     - Regional adjustment factors (zip code based)
     - User's historical pricing (weighted average from past jobs)
     - Market demand adjustment (seasonal, supply chain)
     - Material quality tiers (good/better/best)
   - Generate 3-tier estimates (good/better/best) when applicable
   - Confidence scoring per line item (0-100%)
   - Suggested add-ons based on what's visible (e.g., see old wiring → suggest panel upgrade)
   - Complication warnings (e.g., "possible asbestos in pipe insulation — testing recommended")
   - Permit requirement detection
   - Estimated job duration

3. LEARNING SYSTEM:
   - Collect user corrections: line items added, removed, price changed, quantity adjusted
   - Feedback loop: corrections improve future suggestions for this user
   - Collaborative learning: anonymized corrections improve global model
   - Trade-specific model fine-tuning per user's job history
   - Regional model adaptation based on zip code data density
   - A/B testing framework for model improvements
   - Model versioning and rollback capability
   - Data pipeline: collection → cleaning → labeling → training → validation → deployment
   - Minimum viable training set: 10 corrected estimates per trade category

4. API DESIGN:
   - POST /api/v1/estimates/photo-analyze
     - Input: images (multipart), trade_category, zip_code, user_id
     - Output: { scene_analysis, detected_items[], suggested_line_items[], confidence_score, warnings[], estimated_duration, permit_requirements[] }
   - Async processing with webhook callback for complex analysis
   - Rate limiting: per plan tier
   - Caching: identical image hash returns cached result
   - Latency target: < 8 seconds end-to-end

5. TRADE-SPECIFIC MODELS:
   For each trade, define the classification taxonomy:
   - Plumbing: leak types, pipe materials, fixture types, drain issues, water heater conditions
   - Electrical: panel conditions, wiring types, outlet/switch conditions, lighting types, code violations
   - HVAC: unit types, condition indicators, ductwork issues, thermostat types, refrigerant systems
   - Painting: surface conditions, prep requirements, surface types, coating failures, area estimation
   - Carpentry: wood conditions, structural elements, trim types, cabinet conditions, deck conditions
   - Roofing: shingle conditions, flashing issues, gutter conditions, structural indicators, material types
   - Landscaping: vegetation conditions, hardscape conditions, grading issues, irrigation components
   - Flooring: subfloor conditions, existing material types, damage patterns, transitions, moisture indicators

DELIVERABLES:
- Complete CV pipeline architecture and implementation (Python/PyTorch)
- Estimation generation engine with pricing logic
- Learning system with feedback collection and model updating
- FastAPI service with all endpoints
- Comprehensive test suite with sample images
- Model training pipeline and scripts
- Performance benchmarks
- Data privacy documentation (what's stored, what's anonymized, retention policy)
```

---

### PROMPT 4: PAYMENT & INVOICING ENGINE

```
You are a Principal Engineer at Apple, ICT Level 7+, with expertise in fintech and payment systems. You are building the complete payment and invoicing engine for FieldForge.

CONTEXT:
- Tradespeople lose thousands annually to slow/missed payments — this engine fixes that
- Must support: tap-to-pay (NFC), card on file, online payment links, ACH, Apple Pay, Google Pay, cash/check logging
- Stripe is the primary payment processor, Square as secondary
- Invoices auto-generate from completed jobs
- Must handle: deposits, partial payments, payment plans, tips, refunds, disputes
- Offline payment recording for cash/check with sync

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. INVOICE ENGINE:
   - Invoice data model: header (business info, customer info, invoice #, dates) + line items + tax + totals + payment terms + notes + photos
   - Auto-generation from job data: time entries → labor line items, materials used → material line items, flat rate → single line item
   - Invoice number generation: configurable format (FF-2026-0001, custom prefix, sequential)
   - Tax calculation engine:
     - Auto-detect tax jurisdiction from service address
     - Support for multi-level tax (state + county + city + special district)
     - Tax exemption support (commercial, government, resale)
     - Tax rate lookup via TaxJar or Avalara API
   - Discount application: percentage, flat amount, line-item level or invoice level
   - Late fee calculation: configurable (percentage or flat, grace period, max amount)
   - Credit memo / refund invoice generation
   - Recurring invoice generation for maintenance contracts
   - Batch invoicing (end of day/week/month)
   - Invoice templates: customizable layout with brand colors, logo, footer
   - PDF generation (server-side with Puppeteer or WeasyPrint)
   - Invoice delivery: SMS link, email with PDF attachment, or both
   - Invoice status: Draft → Sent → Viewed → Partially Paid → Paid → Overdue → Void
   - Overdue detection and automated reminder sequence

2. PAYMENT PROCESSING:
   - Stripe integration:
     - PaymentIntents API for one-time payments
     - SetupIntents for saving cards
     - Stripe Terminal SDK for tap-to-pay (M2 reader + iPhone tap-to-pay)
     - Connect accounts for marketplace model (FieldForge takes % cut)
     - Subscription billing for maintenance plans
     - ACH payments (Stripe ACH)
     - Apple Pay / Google Pay
     - Link (Stripe's one-click checkout)
     - Instant Payouts to tradesperson's bank
   - Square integration:
     - Square Reader SDK for card-present payments
     - Square Web Payments SDK for online
     - Square Invoices API
   - Payment recording:
     - Online card payment → auto-reconcile with invoice
     - Tap-to-pay → auto-reconcile
     - Cash payment → manual entry with receipt generation
     - Check payment → manual entry with check number
     - External payment (Venmo, Zelle) → manual entry with reference
   - Deposit handling: configurable percentage, auto-create deposit invoice on estimate approval
   - Partial payment: apply to oldest outstanding or specific invoice
   - Payment plan: split into installments with scheduled charges
   - Tip handling: optional tip prompt on digital payments, configurable percentages
   - Refund processing: full or partial, back to original payment method
   - Dispute/chargeback: notification, documentation upload, status tracking
   - Surcharge: optionally pass processing fee to customer (with legal compliance check by state)

3. ACCOUNTS RECEIVABLE AUTOMATION:
   - Aging buckets: Current, 1-30 days, 31-60 days, 61-90 days, 90+ days
   - Automated reminder sequence:
     - Day of: "Invoice sent"
     - Day 3: "Friendly reminder"
     - Day 7: "Payment reminder"
     - Day 14: "Overdue notice"
     - Day 30: "Final notice"
     - Day 45: "Collections warning"
   - Each step customizable (timing, message, channel)
   - Customer payment portal: view invoices, pay online, download receipts
   - Statement generation: monthly customer statements
   - Collections: export to collections agency API or generate collections letter
   - Write-off workflow with accounting journal entry

4. FINANCIAL REPORTING:
   - Revenue recognition (accrual vs cash basis toggle)
   - Daily/weekly/monthly revenue summary
   - Payment method breakdown
   - Processing fee analysis
   - Outstanding receivables aging
   - Cash flow projection
   - Sales tax collected by jurisdiction
   - 1099 preparation for subcontractors
   - Payout reconciliation with bank deposits
   - Refund and dispute tracking
   - Export to QuickBooks, Xero (journal entries, invoices, payments, customers)

5. EXPENSE TRACKING:
   - Receipt photo → OCR → auto-categorize
   - Categories aligned with Schedule C (IRS)
   - Mileage tracking (auto from GPS) with IRS rate calculation
   - Material purchase logging with supplier
   - Credit card transaction import (Plaid integration)
   - Recurring expense tracking
   - Reimbursement workflow for technicians
   - Expense approval workflow for crew plans
   - Monthly P&L generation

DELIVERABLES:
- Complete Stripe integration (PaymentIntents, Terminal, Connect, Subscriptions, ACH)
- Square integration (Reader, Web Payments)
- Invoice engine with PDF generation
- Tax calculation service
- AR automation with reminder sequences
- Expense tracking with OCR
- Accounting sync (QuickBooks Online)
- Complete test suite (including payment edge cases: declined cards, insufficient funds, network errors during payment, duplicate charges)
- PCI compliance documentation

CODE STANDARDS:
- Idempotent payment operations (exactly-once semantics)
- Complete audit trail for all financial transactions
- Decimal arithmetic for all money calculations (never floating point)
- Amount stored in cents (integer) to prevent rounding errors
- Double-entry bookkeeping for internal ledger
- All payment operations must be crash-safe (resumable)
- Comprehensive error handling with user-friendly messages
- PCI DSS SAQ-A compliance (no card data touches our servers)
```

---

### PROMPT 5: CUSTOMER CRM & COMMUNICATION ENGINE

```
You are a Principal Engineer at Apple, ICT Level 7+. You are building the customer CRM and communication engine for FieldForge.

CONTEXT:
- Tradespeople manage hundreds of customer relationships through texts and memory
- FieldForge replaces this chaos with a structured CRM that feels as easy as texting
- Communication is primarily SMS-based — customers do NOT install the app
- Must track: customer profiles, property details, equipment registries, full communication history, job history, payment history
- Automated communication sequences for reminders, follow-ups, reviews, and marketing

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. CUSTOMER DATA MODEL:
   - Complete customer entity with all fields from section 4.3.1 of the spec
   - Multi-property support (one customer, many addresses)
   - Equipment registry per property (brand, model, serial, install date, warranty expiry, maintenance schedule)
   - Customer-level settings: preferred communication method, preferred technician, credit terms, tax exempt status
   - Customer segmentation: tags, automatic categorization (new, active, VIP, at-risk, inactive, do-not-service)
   - Customer lifetime value calculation (total revenue - total costs, including acquisition cost)
   - Referral chain tracking (who referred whom)
   - Full-text search across all customer fields
   - Import: CSV, vCard, phone contacts API, Angi export, Housecall Pro export

2. COMMUNICATION ENGINE (TWILIO + SENDGRID):
   - Two-way SMS:
     - Send/receive SMS through FieldForge number (Twilio)
     - Conversation threading per customer
     - MMS support (send/receive photos)
     - Template variables: {{customer_name}}, {{tech_name}}, {{job_date}}, {{job_time}}, {{job_address}}, {{estimate_link}}, {{invoice_link}}, {{review_link}}, {{tracking_link}}, {{payment_link}}
     - Delivery status tracking (sent, delivered, failed, undeliverable)
     - Opt-in/opt-out management (TCPA compliance)
     - Message scheduling
     - Auto-response for after-hours messages
   - Email:
     - Transactional emails (estimates, invoices, receipts, confirmations)
     - HTML templates with brand customization
     - Open/click tracking
     - Bounce handling
     - Unsubscribe management (CAN-SPAM)
   - Push Notifications:
     - For app-using team members only (customers never need the app)
   - Call Logging:
     - Tap-to-call from customer profile
     - Auto-log call timestamp and duration
     - Post-call note prompt
     - Voicemail drop integration

3. AUTOMATED COMMUNICATION SEQUENCES:
   - Appointment lifecycle:
     - Estimate sent → follow up in 3 days if no response
     - Appointment confirmed → reminder 24hr before → reminder 1hr before
     - Technician en route → "On my way" with ETA and tracking link
     - Job complete → summary with photos → invoice/payment link
     - Payment received → receipt → review request (24hr delay)
   - Retention:
     - 30 days post-job: "How's everything working?"
     - 90 days: "Time for a check-up?"
     - Annual: seasonal maintenance reminders
     - Birthday/anniversary (if data available)
     - Win-back: re-engage after 6 months inactive
   - Review:
     - Intercept: private satisfaction check first
     - If satisfied: direct to Google → Yelp → Facebook with deep links
     - If unsatisfied: route to owner/manager for resolution
     - Configurable timing and platform priority
   - Fully configurable: timing, message content, conditions, channels
   - A/B testing for message effectiveness

4. CUSTOMER PORTAL (WEB):
   - Token-based access (no login/password for customers)
   - Unique URL per customer
   - Views: upcoming appointments, job history with photos, invoices, receipts
   - Actions: approve estimates, pay invoices, request new service, update info
   - Chat with assigned technician
   - Document access (warranties, permits, certificates)
   - Mobile-responsive design
   - Branded with tradesperson's business identity
   - Built with SvelteKit 5 + TypeScript strict mode

5. REVIEW MANAGEMENT:
   - Automated review request with configurable delays
   - Multi-platform: Google, Yelp, Facebook, Angi, BBB, Nextdoor
   - Negative review interception (private feedback before public review)
   - Review monitoring: webhook/polling for new reviews across platforms
   - Review response: AI-drafted responses for owner approval
   - Review analytics: average rating trend, review velocity, keyword extraction
   - Review showcase: embed widget for website, display on estimates/proposals

DELIVERABLES:
- Complete CRM data model and repository layer
- Twilio integration (SMS, MMS, call logging, voicemail drop)
- SendGrid integration (transactional email, templates)
- Automation engine with sequence builder
- Customer portal (SvelteKit 5, TypeScript strict, fully responsive)
- Review management system with multi-platform support
- Complete test suite including TCPA compliance tests
- Communication analytics dashboard

CODE STANDARDS:
- TCPA compliance: double opt-in, instant opt-out, quiet hours (before 8am, after 9pm customer's local time)
- CAN-SPAM compliance: physical address, unsubscribe link, honest subject lines
- Rate limiting: Twilio's rate limits respected with queue and backpressure
- Idempotent message delivery: prevent duplicate sends on retry
- Complete audit trail for all customer communications
- Encryption for sensitive customer data at rest
- GDPR: data export and deletion capabilities
```

---

### PROMPT 6: INVENTORY, FLEET & EQUIPMENT MANAGEMENT

```
You are a Principal Engineer at Apple, ICT Level 7+. You are building the inventory, fleet, and equipment management system for FieldForge.

CONTEXT:
- Tradespeople carry materials in their trucks and lose track of what they have
- Each technician's truck is essentially a mobile warehouse
- Materials used on jobs must be tracked for billing accuracy and inventory replenishment
- Fleet vehicles need maintenance tracking, mileage logging, and fuel management
- Tools and equipment need check-out/check-in tracking and maintenance schedules

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. INVENTORY SYSTEM:
   - Multi-location: per-vehicle, per-warehouse, per-storage-room
   - Item data: name, SKU, barcode, description, category, subcategory, unit of measure, min stock level, reorder quantity, cost price, markup percentage, sell price, supplier(s), photo, storage location
   - Real-time stock levels updated when:
     - Materials used on a job (auto-deducted)
     - Purchase order received
     - Transfer between locations
     - Physical count adjustment
     - Return/defect
   - Barcode scanning: camera-based barcode/QR reader for quick actions (add, use, count)
   - Low stock alerts: push notification + dashboard highlight when at or below min stock
   - Auto-generate purchase order at reorder point
   - Inventory valuation: FIFO, LIFO, weighted average cost (configurable)
   - Shrinkage/loss tracking with reason codes
   - Physical inventory count workflow: generate count sheet → count → reconcile differences → adjust
   - Transfer workflow: request → approve → ship → receive (with location updates)
   - Inventory search: by name, SKU, barcode, category, location
   - Inventory history per item: complete transaction log

2. PURCHASE ORDER MANAGEMENT:
   - Create PO from low stock alerts, manual, or job material requirements
   - PO data: supplier, items, quantities, unit costs, shipping, tax, total, expected delivery, notes
   - PO approval workflow (for crew/business plans)
   - Send PO to supplier via email
   - Receive against PO: partial receipt, full receipt, with variance tracking
   - PO status: Draft → Sent → Partially Received → Received → Closed
   - Supplier spend analytics
   - Price comparison across suppliers per item

3. FLEET MANAGEMENT:
   - Vehicle registry: make, model, year, VIN, license plate, color, registration expiry, insurance policy, assigned technician
   - GPS tracking: real-time location when in transit (configurable tracking hours)
   - Mileage tracking: auto from GPS (personal vs business classification)
   - Fuel logging: date, gallons, cost, odometer, station
   - Maintenance scheduling: oil change interval, tire rotation, brake inspection, state inspection, custom intervals
   - Maintenance history: date, type, provider, cost, odometer, notes, receipts
   - Vehicle inspection checklists: pre-trip safety inspection (customizable)
   - Vehicle expense tracking: all costs per vehicle for TCO analysis
   - Vehicle assignment history
   - Accident/incident reporting: date, description, photos, insurance claim number, repair status

4. TOOL & EQUIPMENT MANAGEMENT:
   - Equipment registry: name, category, brand, model, serial number, purchase date, purchase price, warranty expiry, assigned to, location, condition, photo
   - Check-out/check-in workflow with assignment tracking
   - Maintenance schedule per tool (calibration, cleaning, blade replacement, etc.)
   - Maintenance logging
   - Tool condition reporting (good, fair, needs repair, out of service)
   - Depreciation tracking (straight-line or declining balance)
   - Replacement value tracking for insurance
   - Lost/stolen/damaged reporting
   - Rental equipment tracking: vendor, daily rate, checkout date, return date, cost
   - QR code labels: print QR labels for tools, scan to view details or log check-out

DELIVERABLES:
- Complete inventory system with multi-location support
- Barcode scanning integration (camera-based)
- Purchase order workflow
- Fleet management with GPS tracking
- Tool/equipment management with check-out system
- All data models, repositories, and business logic
- API endpoints for all operations
- Mobile UI components for inventory actions (scan, count, transfer, use)
- Complete test suite
- Performance: inventory search < 100ms, barcode scan to result < 500ms
```

---

### PROMPT 7: ANALYTICS, REPORTING & BUSINESS INTELLIGENCE

```
You are a Principal Engineer at Apple, ICT Level 7+, with expertise in data engineering and business intelligence. You are building the analytics and reporting engine for FieldForge.

CONTEXT:
- Tradespeople have zero visibility into their business performance
- Most can't answer: "What's my most profitable service?" or "Which customer brings the most revenue?"
- FieldForge turns every data point into actionable business intelligence
- Must support: real-time dashboards, custom reports, automated digests, industry benchmarking, AI-powered insights

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. DATA WAREHOUSE DESIGN:
   - Star schema optimized for analytical queries
   - Fact tables: fact_jobs, fact_invoices, fact_payments, fact_time_entries, fact_expenses, fact_estimates, fact_communications, fact_reviews
   - Dimension tables: dim_customer, dim_technician, dim_service_type, dim_trade, dim_location, dim_time (date dimension), dim_property_type, dim_payment_method
   - Incremental ETL from operational database (PostgreSQL → analytical store)
   - Materialized views for common aggregations
   - Data retention: raw data 7 years, aggregated indefinitely
   - Query performance: any dashboard query < 2 seconds

2. DASHBOARD SYSTEM:
   - Customizable widget-based dashboard
   - Widget types: KPI card, line chart, bar chart, pie/donut, funnel, table, map heatmap, gauge, sparkline
   - Pre-built dashboards:
     - Executive Overview: revenue, jobs, new customers, outstanding AR, today's schedule
     - Financial: revenue trend, P&L, cash flow, AR aging, expense breakdown
     - Operational: job volume, completion rate, utilization, on-time %, callback rate
     - Customer: new vs returning, satisfaction, LTV, churn risk, geographic distribution
     - Technician: performance comparison, efficiency, revenue per tech, utilization
     - Marketing: lead sources, conversion rates, review metrics, referral tracking
   - Date range selector: today, this week, this month, this quarter, this year, custom range
   - Comparison mode: period over period (this month vs last month, this year vs last year)
   - Filter by: technician, trade, customer type, service area, job type
   - Real-time updates (WebSocket for live data)
   - Mobile-optimized dashboard (swipeable widgets)
   - Dashboard sharing: generate shareable link, export to PDF

3. REPORT BUILDER:
   - Drag-and-drop report designer
   - Data sources: all fact and dimension tables
   - Grouping, filtering, sorting
   - Calculated fields (formulas)
   - Chart visualization options
   - Conditional formatting
   - Table totals and subtotals
   - Cross-tabulation (pivot)
   - Report scheduling: daily, weekly, monthly delivery via email
   - Export: PDF, CSV, Excel
   - Report templates: save and reuse
   - Pre-built reports for every category in section 4.8 of the spec

4. AI-POWERED INSIGHTS:
   - Anomaly detection: flag unusual patterns (sudden revenue drop, spike in cancellations, unexpected expense)
   - Trend analysis: identify emerging patterns in revenue, job types, customer behavior
   - Forecasting: revenue, cash flow, demand by trade/season
   - Customer churn prediction: identify at-risk customers with re-engagement recommendations
   - Pricing optimization: suggest optimal pricing based on market, demand, and win rate
   - Upsell recommendations: per customer based on property profile and service history
   - Natural language queries: "What was my revenue from plumbing jobs in January?"
   - Morning briefing generation: AI-composed daily summary of key metrics and action items
   - Weekly digest: AI-composed weekly email with performance summary, trends, and recommendations

5. INDUSTRY BENCHMARKING:
   - Anonymous data aggregation across FieldForge users
   - Benchmarks by: trade, region, company size
   - Metrics: revenue per employee, average job value, collection period, utilization rate, customer satisfaction
   - Percentile ranking: "You're in the 75th percentile for revenue per job in your trade and region"
   - Improvement suggestions based on benchmark gaps

DELIVERABLES:
- Data warehouse schema (star schema)
- ETL pipeline (incremental)
- Dashboard system with all widget types and pre-built dashboards
- Report builder with drag-and-drop UI
- AI insights engine (anomaly detection, forecasting, NLQ)
- Benchmarking system
- Built with SvelteKit 5 + TypeScript for web dashboard
- Charts: D3.js or Chart.js (performant, accessible)
- Complete test suite including data accuracy validation
- Performance benchmarks: dashboard load < 2s, report generation < 5s
```

---

### PROMPT 8: MOBILE APP — COMPLETE iOS IMPLEMENTATION

```
You are a Principal Engineer at Apple, ICT Level 7+. You are building the complete iOS application for FieldForge using Swift 6 and SwiftUI.

CONTEXT:
- This is a mobile-first app for tradespeople who work all day on their phones
- Must be usable with one thumb, with work gloves, in direct sunlight, in dusty/wet conditions
- Must work fully offline
- Performance is critical — tradespeople have no patience for slow apps
- Battery usage must be minimal over a 10-hour workday
- Target devices: iPhone 13+ (iOS 17+)

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. APP ARCHITECTURE:
   - Swift 6 with strict concurrency checking
   - SwiftUI with MVVM + Coordinator pattern
   - Dependencies via Swift Package Manager (no CocoaPods/Carthage)
   - Module structure:
     - FieldForgeApp (app target)
     - FFCore (models, utilities, extensions)
     - FFData (repositories, sync engine, SQLite)
     - FFNetwork (API client, Stripe SDK, Twilio)
     - FFAuth (authentication, biometrics)
     - FFUI (design system, reusable components)
     - FFJobs (job management, state machine)
     - FFCalendar (scheduling, calendar views)
     - FFEstimates (estimation, AI photo)
     - FFInvoices (invoicing, payments)
     - FFCustomers (CRM)
     - FFInventory (materials, tools)
     - FFFleet (vehicles, GPS)
     - FFReports (analytics, dashboards)
     - FFSettings (configuration)
   - Dependency injection with protocol-based approach
   - Navigation: NavigationStack with programmatic routing
   - Deep linking support (Universal Links)
   - Widget extensions: today's schedule, quick actions
   - Live Activities: current job status, ETA tracking

2. DESIGN SYSTEM:
   - Custom design tokens: colors, typography, spacing, shadows, radii
   - Components (complete implementation):
     - FFButton: primary, secondary, destructive, ghost, icon-only — all with loading state
     - FFTextField: text, number, currency, phone, multiline — with validation
     - FFCard: job card, customer card, invoice card, estimate card
     - FFBadge: status badges with trade-specific colors
     - FFAvatar: user, customer, business — with initials fallback
     - FFCalendarView: day, week, month with custom rendering
     - FFMapView: with job pins, route overlay, technician tracking
     - FFPhotoCapture: camera with watermarking, annotation
     - FFSignatureView: signature capture with undo/clear
     - FFChecklistView: interactive checklist with completion tracking
     - FFTimerView: job timer with start/pause/resume/stop
     - FFChartView: line, bar, pie with interactive tooltips
     - FFSearchBar: with recent searches and voice input
     - FFEmptyState: trade-specific illustrations and CTAs
     - FFLoadingState: skeleton screens per view
     - FFErrorState: retry actions with helpful messaging
     - FFSheet: bottom sheet with snap points
     - FFToast: success, error, info, warning
     - FFTabBar: custom tab bar with notification badges
   - Dark mode: full support
   - Dynamic Type: all text scales properly
   - High contrast: accessible color alternatives
   - Motion: reduced motion support
   - Glove-friendly: minimum 48pt touch targets, generous padding

3. CORE SCREENS (COMPLETE IMPLEMENTATION):
   - Home / Dashboard: today's jobs, KPI cards, quick actions, morning briefing
   - Job List: filterable, searchable, sortable with pull-to-refresh
   - Job Detail: tabbed (details, photos, notes, materials, time, checklist)
   - Job Creation: multi-step or single-screen depending on quick vs detailed
   - Calendar: all 6 views (day, week, month, map, kanban, agenda)
   - Customer List: searchable with alphabet scrubber
   - Customer Detail: tabbed (info, properties, jobs, invoices, communication)
   - Estimate Builder: line items, 3-tier options, photo attachment, send
   - AI Photo Estimate: camera → analysis → suggested line items → adjust → send
   - Invoice Detail: view, send, record payment, void
   - Payment Collection: tap-to-pay, card entry, cash/check log
   - Inventory: vehicle stock, scan barcode, transfer, reorder
   - Map / Route: today's route, navigation, traffic overlay
   - Dispatch Board: technician map, assignment, status (iPad/large screen)
   - Reports: dashboard widgets, report viewer, export
   - Settings: business info, templates, integrations, team, billing
   - Profile: user info, availability, skills, vehicle assignment
   - Notifications: inbox with deep links to relevant screens
   - Onboarding: 30-second flow with trade selection, business setup, payment connect

4. CRITICAL FEATURES:
   - Camera: custom camera overlay with watermark preview, burst mode for documentation
   - GPS: CoreLocation with significant-change monitoring (battery efficient)
   - NFC: tap-to-pay via Stripe Terminal SDK
   - Background sync: BGTaskScheduler for photo upload and data sync
   - Push notifications: Firebase + APNs with notification extensions
   - Biometric auth: Face ID / Touch ID for app lock
   - Voice: SFSpeechRecognizer for voice-to-text job creation
   - Haptics: UIImpactFeedbackGenerator for confirmations, drags, and errors
   - Siri Shortcuts: "What's my next job?" "Start timer for current job"
   - Spotlight: index jobs and customers for system search
   - Share extension: receive shared text/photos → create job
   - Widgets: WidgetKit for home screen schedule widget
   - Live Activities: ActivityKit for active job status on lock screen and Dynamic Island
   - iCloud Keychain: secure storage for gate codes, alarm codes

5. PERFORMANCE REQUIREMENTS:
   - Cold launch to interactive: < 1.5 seconds
   - Screen transitions: < 300ms
   - List scrolling: 60fps with 10,000+ items (lazy loading)
   - Camera launch: < 500ms
   - Photo save: < 200ms (background processing for watermark)
   - Search results: < 100ms for local data
   - Memory footprint: < 150MB active, < 50MB background
   - Battery: < 8% drain per 8-hour active workday with GPS
   - Storage: < 200MB app binary, efficient photo caching strategy
   - Crash-free rate: 99.95%+
   - Network: graceful handling of all connectivity states

DELIVERABLES:
- Complete Xcode project structure with all modules
- Design system with all components
- All screens implemented with real data binding
- Offline mode with sync engine integration
- Camera and photo pipeline
- GPS and navigation integration
- Payment collection (Stripe Terminal)
- Push notifications
- Widget and Live Activity implementations
- Voice input integration
- Complete unit tests and UI tests
- Accessibility audit passing (VoiceOver, Dynamic Type, High Contrast)
- Performance profiled with Instruments (no memory leaks, no main thread blocking)
```

---

### PROMPT 9: MOBILE APP — COMPLETE ANDROID IMPLEMENTATION

```
You are a Principal Engineer at Apple, ICT Level 7+ (applying the same rigor to Android development). You are building the complete Android application for FieldForge using Kotlin and Jetpack Compose.

CONTEXT:
- Same requirements as the iOS app — complete feature parity
- Many tradespeople use mid-range Android phones (Samsung A series, Google Pixel a series)
- Must perform well on devices with 4-6GB RAM
- Android fragmentation: support Android 10+ (API 29+)
- Must work fully offline, battery efficient, sunlight readable, glove-friendly

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. APP ARCHITECTURE:
   - Kotlin with Coroutines and Flow
   - Jetpack Compose with Material 3
   - Architecture: MVVM + Clean Architecture + Repository pattern
   - Multi-module Gradle project:
     - :app (application module)
     - :core:model, :core:data, :core:network, :core:database, :core:common
     - :feature:jobs, :feature:calendar, :feature:customers, :feature:estimates
     - :feature:invoices, :feature:payments, :feature:inventory, :feature:fleet
     - :feature:reports, :feature:settings, :feature:onboarding, :feature:auth
     - :ui:designsystem
   - Dependency injection: Hilt
   - Navigation: Compose Navigation with type-safe routes
   - Build: Gradle KTS with version catalog
   - Deep linking support
   - App widgets: Glance API
   - Persistent notification for active job timer

2. DESIGN SYSTEM (COMPOSE):
   - Complete theme: FieldForgeTheme with colors, typography, shapes
   - Same component set as iOS (section 2 of Prompt 8) implemented in Compose
   - Material 3 design tokens extended with FieldForge brand
   - Adaptive layouts: phone, tablet, foldable
   - Dark mode with Material You dynamic color support
   - Accessibility: TalkBack, font scaling, high contrast

3. CORE SCREENS:
   - Complete feature parity with iOS (section 3 of Prompt 8)
   - Tablet-optimized layouts for dispatch board and reports
   - Foldable support (Samsung Galaxy Z Fold)
   - Landscape mode for map and calendar views

4. CRITICAL FEATURES:
   - Camera: CameraX with custom overlay and watermarking
   - GPS: FusedLocationProviderClient with geofencing
   - NFC: Stripe Terminal SDK for tap-to-pay, plus native NFC for future features
   - Background sync: WorkManager with constraints (battery, network)
   - Push: Firebase Cloud Messaging with notification channels
   - Biometric: BiometricPrompt API
   - Voice: Android Speech Recognition API
   - App widget: Glance with today's schedule
   - Quick Settings tile: start/stop job timer
   - Share target: receive shared text/photos to create job
   - Android Auto: basic job navigation support

5. PERFORMANCE:
   - Same targets as iOS (section 5 of Prompt 8)
   - ProGuard/R8 optimized release build
   - Baseline profiles for cold start optimization
   - Memory leak detection with LeakCanary (debug)
   - Strict mode enabled in debug builds
   - ANR rate: < 0.1%

DELIVERABLES:
- Complete Android project with multi-module setup
- Design system with all Compose components
- All screens with full data binding
- Offline mode with sync engine
- Camera, GPS, NFC, Voice integrations
- WorkManager background sync
- Firebase Cloud Messaging
- Biometric authentication
- App widget and Quick Settings tile
- Complete unit tests (JUnit 5 + MockK) and UI tests (Compose Test)
- Accessibility audit passing
- Performance profiled with Android Profiler
- Baseline profiles generated
- Google Play Store listing assets prepared
```

---

### PROMPT 10: WEB ADMIN DASHBOARD & CUSTOMER PORTAL

```
You are a Principal Engineer at Apple, ICT Level 7+. You are building the web admin dashboard and customer portal for FieldForge using SvelteKit 5, Svelte 5 runes, and TypeScript strict mode.

CONTEXT:
- The web dashboard is used by office managers, business owners, and dispatchers on desktop/tablet
- The customer portal is the customer-facing web experience (no app install for customers)
- Both must be fast, responsive, and production-grade
- SvelteKit 5 with Svelte 5 runes ($state, $derived, $effect, $props), snippets over slots
- TypeScript strict mode with zero errors/warnings
- pnpm as package manager

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. ADMIN DASHBOARD:
   - Authentication: Supabase Auth (email/password, Google, magic link)
   - Role-based access: Owner, Admin, Office Manager, Dispatcher
   - Sidebar navigation with collapsible groups
   - Global search (customers, jobs, invoices — instant, Cmd/Ctrl+K)
   - Screens:
     - Dashboard: KPI widgets, today's summary, alerts, charts
     - Jobs: table with advanced filters, bulk actions, detail slideover
     - Calendar: full-featured scheduling with all 6 views
     - Dispatch: real-time technician map with job assignment
     - Customers: searchable table, detail page with full history
     - Estimates: list, builder, send
     - Invoices: list, detail, payment recording, batch operations
     - Payments: transaction list, reconciliation, payout tracking
     - Inventory: stock levels, PO management, transfer tracking
     - Fleet: vehicle list, GPS map, maintenance schedule
     - Team: member management, availability, performance
     - Reports: interactive dashboards, report builder, export
     - Marketing: review management, campaign builder, referral tracking
     - Settings: business info, templates, integrations, billing, automation rules
   - Real-time updates via WebSocket (new job, payment received, status change)
   - Keyboard shortcuts for power users
   - Print-friendly layouts for reports and invoices

2. CUSTOMER PORTAL:
   - Token-based access (unique URL per customer, no login required)
   - Optional login for returning customers (email + magic link)
   - Screens:
     - Home: upcoming appointments, recent activity
     - Appointments: list with details, reschedule/cancel options
     - Estimates: view, compare options, approve with e-signature, decline with reason
     - Invoices: view, pay online (Stripe Elements), download receipt
     - Job History: with photos, notes, warranty info
     - Documents: contracts, warranties, permits, certificates
     - New Service Request: form with photo upload
     - Chat: real-time messaging with assigned technician
     - Profile: update contact info, communication preferences
   - Branded with tradesperson's business identity (logo, colors)
   - Mobile-responsive (customers use phones primarily)
   - Multi-language support (English, Spanish)

3. TECHNICAL REQUIREMENTS:
   - SvelteKit 5 with adapter-auto (Vercel, Cloudflare, or Node)
   - Svelte 5 runes exclusively ($state, $derived, $effect, $props)
   - Snippets over slots for component composition
   - TypeScript strict mode — zero errors, zero warnings, zero `any` types
   - pnpm for package management
   - Styling: Tailwind CSS 4 with custom design tokens
   - Icons: Iconify with Phosphor or Carbon icon sets (NO Lucide)
   - Charts: D3.js or Layerchart (Svelte-native)
   - Maps: MapLibre GL JS with Mapbox tiles
   - Tables: TanStack Table (Svelte adapter)
   - Forms: Superforms + Zod validation
   - Real-time: Socket.io or native WebSocket
   - API client: typed fetch wrapper with error handling
   - Image optimization: Sharp for server-side, lazy loading for client
   - SEO: proper meta tags, OG tags, structured data
   - Performance: < 1.5s LCP, < 100ms FID, < 0.1 CLS
   - Bundle: code splitting per route, < 150KB initial JS
   - Testing: Vitest for unit, Playwright for E2E
   - Accessibility: WCAG 2.1 AA compliance
   - CSP headers, CORS config, rate limiting

4. DESIGN SYSTEM (SVELTE 5):
   - Complete component library:
     - Button, IconButton, ButtonGroup
     - Input, Select, Checkbox, Radio, Switch, Slider, DatePicker, TimePicker
     - Table with sorting, filtering, pagination, row selection, bulk actions
     - Card, Panel, Modal, Drawer, Sheet, Popover, Tooltip
     - Tabs, Accordion, Breadcrumb, Pagination
     - Badge, Tag, Avatar, Indicator
     - Toast, Alert, Banner
     - Sidebar, Navbar, CommandPalette
     - Chart (line, bar, pie, donut, area, sparkline)
     - Map with markers, routes, heatmaps
     - FileUpload, PhotoCapture, SignaturePad
     - Skeleton, Spinner, Progress
     - EmptyState, ErrorBoundary
   - All components: typed $props, proper $state management, accessible, dark mode support
   - Storybook-like documentation page with live examples

DELIVERABLES:
- Complete SvelteKit 5 project structure
- Design system with all components
- Admin dashboard with all screens
- Customer portal with all screens
- Authentication and authorization
- Real-time updates
- Complete Vitest unit tests
- Playwright E2E test suite
- Accessibility audit passing
- Lighthouse scores: 95+ across all categories
- Deployment configuration
```

---

### PROMPT 11: API & BACKEND SERVICES

```
You are a Principal Engineer at Apple, ICT Level 7+. You are building the complete backend API and services for FieldForge.

CONTEXT:
- The backend serves iOS, Android, and Web clients
- Must handle: authentication, authorization, CRUD for all entities, real-time events, file uploads, payment processing, AI inference proxy, third-party integrations
- Performance and reliability are critical — tradespeople depend on this for their livelihood
- Choose: Rust/Axum (performance) OR Node.js/Fastify (speed of development) — implement in the one you recommend for this use case

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. API DESIGN:
   - RESTful API with OpenAPI 3.1 specification
   - GraphQL API for flexible client queries (optional, for admin dashboard)
   - Versioned: /api/v1/
   - Resources: auth, users, teams, jobs, customers, properties, equipment, estimates, invoices, payments, time_entries, photos, documents, inventory, vehicles, fleet, reports, notifications, integrations, subscriptions, webhooks
   - Consistent response format: { data, meta, errors }
   - Pagination: cursor-based for lists
   - Filtering: query parameter syntax (?status=active&trade=plumbing&created_after=2026-01-01)
   - Sorting: ?sort=created_at:desc,name:asc
   - Field selection: ?fields=id,title,status,customer.name
   - Batch operations: POST /api/v1/jobs/batch { operations: [{ method, path, body }] }
   - Rate limiting: per plan tier (Solo: 100/min, Crew: 500/min, Business: 2000/min)
   - API key + OAuth 2.0 authentication
   - Comprehensive error codes with actionable messages

2. DATABASE:
   - PostgreSQL 16 with:
     - Complete schema for all entities (normalized, 3NF minimum)
     - Row Level Security (RLS) policies per tenant
     - Multi-tenant architecture: schema-per-tenant or shared with tenant_id discriminator
     - Indexes: B-tree for lookups, GIN for full-text search, BRIN for time-series data, GiST for geospatial
     - Partitioning: time-based for large tables (jobs, time_entries, photos)
     - Materialized views for reporting aggregations
     - Database migrations with up/down scripts
     - Seeding scripts for development data
     - Connection pooling (PgBouncer)
     - Read replicas for reporting queries
     - Point-in-time recovery configuration
     - Backup automation

3. BACKGROUND JOBS:
   - Job queue: Redis + BullMQ (Node) or Tokio tasks (Rust)
   - Jobs:
     - Invoice PDF generation
     - Photo processing (resize, watermark, thumbnail)
     - Email sending
     - SMS sending
     - Push notification dispatch
     - AI photo analysis orchestration
     - Payment processing webhooks
     - Sync conflict resolution
     - Report generation
     - Data export
     - Scheduled reminders
     - Recurring job generation
     - Accounting sync (QuickBooks, Xero)
     - Review monitoring
   - Dead letter queue for failed jobs
   - Job retry with exponential backoff
   - Job priority levels
   - Monitoring and alerting on queue depth

4. REAL-TIME:
   - WebSocket server for live updates
   - Events: job_status_changed, payment_received, new_message, technician_location_updated, schedule_changed, inventory_alert
   - Room-based: per-user, per-team, per-job
   - Presence: online/offline status for team members
   - Delivery guarantee: at-least-once with client deduplication

5. FILE STORAGE:
   - S3-compatible object storage (AWS S3 or Cloudflare R2)
   - Photo upload pipeline:
     - Client uploads to pre-signed URL
     - Server processes: validate, generate thumbnail, apply watermark, extract EXIF
     - Store: original, thumbnail (200px), medium (800px), large (1600px)
     - CDN distribution for fast access
   - Document storage: contracts, permits, licenses, receipts
   - Storage quota enforcement per plan tier
   - File cleanup for deleted resources

6. INTEGRATIONS SERVICE:
   - Stripe: complete integration (payments, subscriptions, connect, terminal, invoicing)
   - QuickBooks Online: OAuth2 + two-way sync (customers, invoices, payments, expenses)
   - Twilio: SMS, MMS, call tracking, voicemail
   - SendGrid: transactional and marketing email
   - Google Maps: geocoding, directions, places
   - TaxJar/Avalara: tax calculation and filing
   - Plaid: bank account connection for expense import
   - Zapier: webhook triggers and actions
   - Integration health monitoring and retry logic

7. SECURITY:
   - Authentication: JWT with refresh tokens, OAuth 2.0 for third-party
   - Authorization: RBAC with permissions matrix
   - Input validation: Zod (Node) or serde validation (Rust) on every endpoint
   - SQL injection: parameterized queries only
   - XSS: output encoding, CSP headers
   - CSRF: token-based protection
   - Rate limiting: per IP, per user, per API key
   - Request signing for webhooks
   - Encryption: AES-256 at rest, TLS 1.3 in transit
   - PCI compliance: no card data stored, Stripe handles all card processing
   - Audit logging: every write operation logged with user, timestamp, IP, changes
   - Secrets management: environment variables, never in code
   - Dependency scanning: automated CVE checks

DELIVERABLES:
- Complete API implementation with all endpoints
- OpenAPI 3.1 specification
- Database schema with migrations
- Background job processors
- WebSocket server
- File storage service
- Integration services (Stripe, QuickBooks, Twilio, SendGrid)
- Authentication and authorization system
- Complete test suite (unit, integration, API)
- Docker Compose for local development
- CI/CD pipeline configuration
- Monitoring and alerting setup
- API documentation
- Load test scripts and performance benchmarks
```

---

### PROMPT 12: AI/ML SERVICES — COMPLETE PIPELINE

```
You are a Principal Engineer at Apple, ICT Level 7+, with deep ML/AI expertise. You are building the complete AI/ML service layer for FieldForge.

CONTEXT:
- FieldForge's AI features are the key differentiator: photo-based estimation, smart scheduling, predictive analytics, NLP job creation, and business intelligence
- These services must be accurate, fast, and continuously improving
- Python with FastAPI for the ML service layer
- Models must be optimized for production: latency, throughput, cost

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. PHOTO ESTIMATION SERVICE:
   - Model: Fine-tuned vision model (CLIP + custom heads, or Florence-2, or Qwen-VL)
   - Pipeline: image upload → preprocessing → scene classification → object detection → damage assessment → line item generation → pricing → response
   - Scene categories: 50+ covering all trades
   - Object detection: 500+ object classes across trades
   - Damage severity: 4-level classification (minor, moderate, major, critical)
   - Line item mapping: CV outputs → trade-specific catalog items
   - Pricing engine: base price × regional factor × user history weight × demand factor
   - Confidence scoring per line item
   - Multi-image analysis (combine findings from multiple photos)
   - Latency: < 8 seconds end-to-end
   - Fallback: if AI confidence < 40%, return "manual estimation recommended" with detected context

2. SMART SCHEDULING SERVICE:
   - Algorithm: Constraint satisfaction + genetic algorithm for route optimization
   - Inputs: jobs (time windows, duration, priority, location, skills required), technicians (availability, skills, location, vehicle capacity), traffic patterns
   - Constraints: hard (working hours, required skills) and soft (preferred technician, minimize travel)
   - Output: optimized schedule with route and estimated times
   - Re-optimization: insert/remove jobs without full recalculation
   - Performance: optimize 50 jobs across 5 technicians in < 2 seconds

3. NLP SERVICE:
   - Voice-to-job: speech text → structured job data (customer, address, description, date/time)
   - SMS/email-to-job: parse inbound messages into job requests
   - Natural language search: "show me all plumbing jobs from last month that were over $500"
   - Review response generation: context-aware draft responses to customer reviews
   - Business Q&A: "What was my revenue last quarter?" → SQL query → natural language response
   - Model: Fine-tuned LLM or GPT-4/Claude API with structured output

4. PREDICTIVE ANALYTICS:
   - Revenue forecasting: time series model (Prophet or custom LSTM)
   - Customer churn prediction: classification model with feature engineering from job/communication patterns
   - Job duration prediction: regression model using job type, complexity, technician history
   - Demand forecasting: predict job volume by trade and season
   - Equipment failure prediction: based on age, usage, maintenance history
   - Lead quality scoring: predict conversion probability from lead attributes

5. ML PIPELINE:
   - Data collection: structured logging of all model inputs and user corrections
   - Feature store: centralized feature computation and storage
   - Training pipeline: automated retraining on schedule or data drift detection
   - Model versioning: MLflow or similar
   - A/B testing framework: test new models against production
   - Monitoring: model accuracy metrics, latency, drift detection
   - GPU inference: NVIDIA T4 or A10G for vision models
   - Model optimization: ONNX export, quantization, batching
   - Cost optimization: spot instances for training, reserved for inference
   - Privacy: no raw customer data in training sets, differential privacy for aggregations

DELIVERABLES:
- FastAPI service with all ML endpoints
- Photo estimation pipeline (model training scripts + inference service)
- Scheduling optimization algorithm
- NLP service (voice-to-job, message parsing, natural language search)
- Predictive analytics models (forecasting, churn, duration, demand)
- ML pipeline infrastructure (data collection, training, deployment, monitoring)
- Complete test suite with accuracy benchmarks
- API documentation
- Cost analysis per inference call
- Model performance dashboards
```

---

### PROMPT 13: DEVOPS, INFRASTRUCTURE & DEPLOYMENT

```
You are a Principal Engineer at Apple, ICT Level 7+, with expertise in cloud infrastructure and DevOps. You are building the complete infrastructure for FieldForge.

CONTEXT:
- FieldForge must be highly available (99.99% uptime target for Business/Enterprise)
- Must scale from 0 to 100K+ concurrent users
- Multi-region for low latency across the US
- Cost-efficient at early stage, scalable for growth
- Security-first: SOC 2 Type II, PCI DSS, GDPR ready

REQUIREMENTS — COMPLETE IMPLEMENTATION:

1. INFRASTRUCTURE (AWS):
   - VPC: multi-AZ with public, private, and isolated subnets
   - Compute: ECS Fargate for API services, Lambda for event processing
   - Database: RDS PostgreSQL Multi-AZ with read replicas
   - Cache: ElastiCache Redis (cluster mode)
   - Storage: S3 with lifecycle policies (hot → warm → cold)
   - CDN: CloudFront with edge caching for static assets and photos
   - Queue: SQS for async jobs, SNS for fan-out notifications
   - Search: OpenSearch for full-text search
   - ML: SageMaker endpoints for AI inference
   - Networking: ALB with WAF, Route 53 for DNS
   - Secrets: AWS Secrets Manager
   - Monitoring: CloudWatch + Grafana + Sentry

2. CI/CD:
   - GitHub Actions pipelines:
     - On PR: lint, type-check, unit tests, integration tests, security scan
     - On merge to main: build, test, deploy to staging
     - On release tag: deploy to production with canary rollout
   - Mobile CI/CD:
     - iOS: Xcode Cloud or Fastlane + GitHub Actions → TestFlight → App Store
     - Android: Gradle + GitHub Actions → Internal Testing → Play Store
   - Database migrations: automated with rollback capability
   - Feature flags: LaunchDarkly or Unleash for gradual rollout
   - Blue-green deployments for zero-downtime
   - Automated rollback on error rate spike

3. MONITORING & OBSERVABILITY:
   - Metrics: request latency (p50, p95, p99), error rate, throughput, queue depth, DB connections
   - Logs: structured JSON logging, centralized in CloudWatch/Datadog
   - Tracing: distributed tracing with OpenTelemetry
   - Alerting: PagerDuty integration with escalation policies
   - Dashboards: infrastructure, application, business metrics
   - Synthetic monitoring: health checks from multiple regions
   - Real User Monitoring: mobile and web performance tracking
   - Error tracking: Sentry with release correlation
   - SLA monitoring: uptime calculation per tier

4. SECURITY:
   - WAF rules: OWASP Top 10 protection
   - DDoS protection: AWS Shield Standard + Advanced
   - Vulnerability scanning: automated dependency and container scanning
   - Penetration testing: quarterly (Cobalt, HackerOne)
   - Access control: IAM with least-privilege, MFA for all humans
   - Audit trail: CloudTrail + custom application audit logs
   - Encryption: KMS for key management, envelope encryption
   - Compliance: SOC 2 Type II controls documented and evidenced
   - Incident response: runbook for common incidents
   - Backup testing: monthly restore drills

5. COST OPTIMIZATION:
   - Right-sizing: auto-scaling based on demand
   - Reserved instances for predictable workloads
   - Spot instances for ML training
   - S3 Intelligent-Tiering for photo storage
   - CDN caching to reduce origin requests
   - Database query optimization monitoring
   - Monthly cost review and optimization
   - Budget alerts at 80% and 100% thresholds
   - Cost allocation tags for per-service tracking

DELIVERABLES:
- Terraform/CDK infrastructure as code (complete)
- CI/CD pipeline configurations (GitHub Actions)
- Docker configurations for all services
- Monitoring and alerting setup
- Security configuration and documentation
- Disaster recovery plan and runbooks
- Cost estimation at 1K, 10K, 100K user scales
- Load testing scripts and results
- SOC 2 control mapping document
```

---

## IMPLEMENTATION SEQUENCE

### Phase 1: Foundation (Weeks 1-6)
1. Infrastructure setup (Prompt 13)
2. Database schema and sync engine (Prompt 1)
3. API core with auth and basic CRUD (Prompt 11)

### Phase 2: Core Product (Weeks 7-14)
4. Job management and scheduling (Prompt 2)
5. Customer CRM and communication (Prompt 5)
6. Estimating and invoicing (Prompt 4)

### Phase 3: Mobile (Weeks 10-20, parallel)
7. iOS app (Prompt 8)
8. Android app (Prompt 9)

### Phase 4: Web (Weeks 12-18, parallel)
9. Admin dashboard and customer portal (Prompt 10)

### Phase 5: AI & Intelligence (Weeks 16-24)
10. AI photo estimation (Prompt 3)
11. Smart scheduling and predictive analytics (Prompt 12)

### Phase 6: Advanced Features (Weeks 20-28)
12. Inventory and fleet management (Prompt 6)
13. Analytics and reporting (Prompt 7)

### Phase 7: Polish & Launch (Weeks 28-32)
14. Performance optimization
15. Security audit and penetration testing
16. App Store and Play Store submission
17. Beta launch with 100 tradespeople
18. Public launch

---

## APPENDIX: KEY METRICS FOR SUCCESS

| Metric | Year 1 Target | Year 2 Target | Year 3 Target |
|--------|:---:|:---:|:---:|
| Registered Users | 10,000 | 50,000 | 200,000 |
| Paying Subscribers | 2,000 | 15,000 | 75,000 |
| MRR | $70K | $525K | $2.6M |
| ARR | $840K | $6.3M | $31.2M |
| Churn Rate | < 5%/mo | < 3%/mo | < 2%/mo |
| NPS | 50+ | 60+ | 70+ |
| App Store Rating | 4.5+ | 4.7+ | 4.8+ |
| Jobs Processed/mo | 100K | 1M | 10M |
| Payment Volume/mo | $5M | $50M | $500M |
| Uptime | 99.9% | 99.95% | 99.99% |

---

*This document is the complete specification and implementation guide for FieldForge. Every feature listed is required for industry leadership. The AI prompts above will produce production-grade implementations when executed by a Principal Engineer-level AI assistant.*

*Built for 10-year longevity. No shortcuts. No compromises.*
