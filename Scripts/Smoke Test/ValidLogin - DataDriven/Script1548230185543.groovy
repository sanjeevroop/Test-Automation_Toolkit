import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.checkpoint.CheckpointFactory as CheckpointFactory
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as MobileBuiltInKeywords
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testcase.TestCaseFactory as TestCaseFactory
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testdata.TestDataFactory as TestDataFactory
import com.kms.katalon.core.testobject.ObjectRepository as ObjectRepository
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WSBuiltInKeywords
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUiBuiltInKeywords
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW

WebUI.openBrowser(findTestData('Environment').getValue('URL', 1))

WebUI.maximizeWindow()

WebUI.navigateToUrl(findTestData('Environment').getValue('URL', 1))

WebUI.setText(findTestObject('Object Repository/Login/input_Email'), 'purmanand.roopnah@sdworx.com')

WebUI.setText(findTestObject('Object Repository/Login/input_Password'), 'Manish_2505')

WebUI.click(findTestObject('Object Repository/Login/input_btn btn-default'))

WebUI.selectOptionByLabel(findTestObject('Dropdown Lists/Select Organisation/Select Org'), 'Panasonic Mauritius', true, 
    FailureHandling.CONTINUE_ON_FAILURE)

WebUI.verifyElementPresent(findTestObject('Logout/Logout_dropdown'), 10, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Logout/Logout_dropdown'))

WebUI.verifyElementPresent(findTestObject('Logout/a_Logout'), 10, FailureHandling.CONTINUE_ON_FAILURE)

WebUI.click(findTestObject('Logout/a_Logout'))

WebUI.closeBrowser()

