import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

WebUI.callTestCase(findTestCase('Smoke Test/ValidLogin'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Navigations/Navigation - Basic Details'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Navigations/Navigation - Tax Groups'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Navigations/Navigation - Pay Groups'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Navigations/Navigation - Department'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Navigations/Navigation - Cost Code'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Navigations/Navigation - Payroll Disbursements'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Navigations/Navigation - Personal Data Codes'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Navigations/Navigation - Payroll Class'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Navigations/Navigation - Payroll Category'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Navigations/Navigation - Holiday Rate Calculation'), [:], FailureHandling.STOP_ON_FAILURE)

WebUI.callTestCase(findTestCase('Logout/Logout'), [:], FailureHandling.STOP_ON_FAILURE)

