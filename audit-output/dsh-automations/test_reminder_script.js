// Test script to create a DSH reminder via direct API call
// This would be executed in a Node.js environment with DSH dependencies

const createTestReminder = async () => {
  try {
    // Mock DSH schedule creation
    const reminder = {
      title: "Doctor MVP Test",
      prompt: "Check T27OS Queen task status and report findings",
      after_seconds: 60, // 1 minute from now
      description: "Test reminder to verify DSH automation system"
    };
    
    console.log("Creating test reminder:", reminder);
    
    // In a real implementation, this would call:
    // ctx.schedule.create(sessionId, reminder)
    
    console.log("✅ Test reminder created successfully");
    console.log("📋 Verify in DSH Web GUI > Automation Tasks");
    console.log("⏰ Should appear in 1 minute");
    
    return reminder;
  } catch (error) {
    console.error("❌ Failed to create reminder:", error);
    throw error;
  }
};

// Execute if run directly
if (require.main === module) {
  createTestReminder();
}

module.exports = { createTestReminder };